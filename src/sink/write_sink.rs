use std::{io::Write, mem, sync::atomic::Ordering};

use atomic::Atomic;

use crate::{
    formatter::{Formatter, FullFormatter},
    prelude::*,
    Error, Record, Result, Sink, StringBuf,
};

/// A sink with a generic that implements [`Write`] trait as the target
pub struct WriteSink<W>
where
    W: Write + Send,
{
    level_filter: Atomic<LevelFilter>,
    formatter: spin::RwLock<Box<dyn Formatter>>,
    target: spin::Mutex<W>,
}

impl<W> WriteSink<W>
where
    W: Write + Send,
{
    /// Constructs a `WriteSink`.
    pub fn new(target: W) -> Self {
        Self {
            level_filter: Atomic::new(LevelFilter::All),
            formatter: spin::RwLock::new(Box::new(FullFormatter::new())),
            target: spin::Mutex::new(target),
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

        self.target
            .lock()
            .write_all(string_buf.as_bytes())
            .map_err(Error::WriteRecord)?;

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.target.lock().flush().map_err(Error::FlushBuffer)
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
        if let Err(err) = self.target.lock().flush() {
            // Sinks do not have an error handler, because it would increase complexity and
            // the error is not common. So currently users cannot handle this error by
            // themselves.
            crate::default_error_handler("WriteSink", Error::FlushBuffer(err));
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
