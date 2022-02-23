use std::{
    io::Write,
    mem,
    sync::{self, atomic::Ordering},
};

use atomic::Atomic;

use crate::{
    formatter::{Formatter, FullFormatter},
    prelude::*,
    Error, Record, Result, Sink, StringBuf,
};

/// A sink that writes log messages into an arbitrary `impl Write` object.
///
/// # Performance Notice
///
/// Since `WriteSink` can write into any `impl Write` objects, the assumptions made on the underlying `impl Write`
/// object is very weak and this does impact performance. You should use other sinks or implement your own sinks
/// whenever possible. `WriteSink` is your last resort if no other sinks meet your requirement.
///
/// If you want to log into a file, use [`FileSink`] or [`RotatingFileSink`] instead.
///
/// If you want to log into the standard streams, use [`StdStreamSink`] instead.
///
/// [`FileSink`]: crate::sink::FileSink
/// [`RotatingFileSink`]: crate::sink::RotatingFileSink
/// [`StdStreamSink`]: crate::sink::StdStreamSink
pub struct WriteSink<W>
where
    W: Write + Send,
{
    level_filter: Atomic<LevelFilter>,
    formatter: spin::RwLock<Box<dyn Formatter>>,
    target: sync::Mutex<W>,
}

impl<W> WriteSink<W>
where
    W: Write + Send,
{
    /// Constructs a `WriteSink` that writes log messages into the given `impl Write` object.
    pub fn new(target: W) -> Self {
        Self {
            level_filter: Atomic::new(LevelFilter::All),
            formatter: spin::RwLock::new(Box::new(FullFormatter::new())),
            target: sync::Mutex::new(target),
        }
    }
}

impl<W> Sink for WriteSink<W>
where
    W: Write + Send,
{
    fn log(&self, record: &Record) -> Result<()> {
        if !self.should_log(record.level()) {
            return Ok(());
        }

        let mut string_buf = StringBuf::new();
        self.formatter.read().format(record, &mut string_buf)?;

        let mut locked_target = self
            .target
            .lock()
            .map_err(|err| Error::LockMutex(format!("{}", err)))?;
        locked_target
            .write_all(string_buf.as_bytes())
            .map_err(Error::WriteRecord)?;

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.target
            .lock()
            .map_err(|err| Error::LockMutex(format!("{}", err)))
            .and_then(|mut locked_target| locked_target.flush().map_err(Error::FlushBuffer))
    }

    fn level_filter(&self) -> LevelFilter {
        self.level_filter.load(Ordering::Relaxed)
    }

    fn set_level_filter(&self, level_filter: LevelFilter) {
        self.level_filter.store(level_filter, Ordering::Relaxed);
    }

    fn swap_formatter(&self, mut formatter: Box<dyn Formatter>) -> Box<dyn Formatter> {
        mem::swap(&mut *self.formatter.write(), &mut formatter);
        formatter
    }
}

impl<W> Drop for WriteSink<W>
where
    W: Write + Send,
{
    fn drop(&mut self) {
        match self.target.lock() {
            Ok(mut locked_target) => {
                if let Err(err) = locked_target.flush() {
                    // Sinks do not have an error handler, because it would increase complexity and
                    // the error is not common. So currently users cannot handle this error by
                    // themselves.
                    crate::default_error_handler("WriteSink", Error::FlushBuffer(err));
                }
            }
            Err(err) => {
                crate::default_error_handler("WriteSink", Error::LockMutex(format!("{}", err)));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_utils::*, utils};

    use std::{fs, path::PathBuf, sync::Arc};

    use once_cell::sync::Lazy;

    static BASE_LOGS_PATH: Lazy<PathBuf> = Lazy::new(|| {
        let path = TEST_LOGS_PATH.join("write_sink");
        fs::create_dir_all(&path).unwrap();
        path
    });

    #[test]
    fn validation() {
        let file_path = BASE_LOGS_PATH.join("validation.txt");

        let file = utils::open_file(&file_path, true).unwrap();

        let sink = Arc::new(WriteSink::new(file));
        sink.set_formatter(Box::new(NoModFormatter::new()));
        let logger = test_logger_builder()
            .sink(sink)
            .level_filter(LevelFilter::All)
            .build();

        info!(logger: logger, "hello WriteSink");

        assert_eq!(fs::read_to_string(file_path).unwrap(), "hello WriteSink");
    }
}
