//! A fast and combinable Rust logging crate.
//!
//! It is inspired by the C++ logging library [spdlog], so if you are familiar
//! with C++ `spdlog`, you should be able to get started with this crate quite
//! easily. Of course, there are some differences, you can see [Significant
//! differences from C++ spdlog](#significant-differences-from-c-spdlog) below.
//!
//! # Getting started
//!
//! Add this to `Cargo.toml`:
//! ```toml
//! [dependencies]
//! spdlog-rs = "0.2"
//! ```
//!
//! `spdlog-rs` is out-of-the-box, it has a default logger, so users can output
//! logs to terminal by default without any configuration. For more details
//! about the default logger, please read the documentation of
//! [`default_logger`].
//!
//! The basic use of this crate is through these logging macros: [`trace!`],
//! [`debug!`], [`info!`], [`warn!`], [`error!`], [`critical!`] and [`log!`],
//! where [`critical!`] represents the most severe log messages and [`trace!`]
//! the most verbose. Each of these macros accept format strings similarly to
//! [`println!`]. All log macros and common types are already under [`prelude`]
//! module.
//!
//! [`Logger`] and [`Sink`] are the most important components of `spdlog-rs`.
//! Make sure to read their documentation. In short, a logger contains a
//! combination of sinks, and sinks implement writing log messages to actual
//! targets.
//!
//! ## Examples
//!
//! ```
//! use spdlog::prelude::*;
//!
//! info!("hello, {}", "world");
//! ```
//!
//! For more examples, see [./examples] directory.
//!
//! ## Help
//!
//! If you have any questions or need help while using this crate, feel free to
//! [open a discussion]. For feature requests or bug reports, please [open an
//! issue].
//!
//! # Compatible with log crate
//!
//! This is optional and is controlled by crate feature `log`.
//!
//! The compatibility with [log crate] is mainly through a proxy layer
//! [`LogCrateProxy`]. Call [`init_log_crate_proxy`] function to enable the
//! proxy layer, and all logs from [log crate] will be handled by it. You can
//! use it to output [log crate] logs of upstream dependencies or to quickly
//! migrate from [log crate] for your projects.
//!
//! [`LogCrateProxy`] forwards all logs from [log crate] to [`default_logger`]
//! by default, you can call [`log_crate_proxy()`] to get a reference to this
//! proxy to configure it.
//!
//! ## Examples
//!
//! See [./examples] directory.
//!
//! # Configured via environment variable
//!
//! Users can optionally configure the level filter of loggers via the
//! environment variable `SPDLOG_RS_LEVEL`.
//!
//! For more details, see the documentation of [`init_env_level`].
//!
//! # Compile time filters
//!
//! Log levels can be statically disabled at compile time via Cargo features.
//! Log invocations at disabled levels will be skipped and will not even be
//! present in the resulting binary. This level is configured separately for
//! release and debug builds. The features are:
//!
//!  - `level-off`
//!  - `level-critical`
//!  - `level-error`
//!  - `level-warn`
//!  - `level-info`
//!  - `level-debug`
//!  - `level-trace`
//!  - `release-level-off`
//!  - `release-level-critical`
//!  - `release-level-error`
//!  - `release-level-warn`
//!  - `release-level-info`
//!  - `release-level-debug`
//!  - `release-level-trace`
//!
//! These features control the value of the `STATIC_LEVEL_FILTER` constant. The
//! logging macros check this value before logging a message. By default, no
//! levels are disabled.
//!
//! For example, a crate can disable trace level logs in debug builds and trace,
//! debug, and info level logs in release builds with
//! `features = ["level-debug", "release-level-warn"]`.
//!
//! # Crate Feature Flags
//!
//! The following crate feature flags are available in addition to the filters.
//! They are configured in your `Cargo.toml`.
//!
//!  - `source-location` allows recording the source location of each log. When
//!    it is enabled the default formatter [`FullFormatter`] will always format
//!    the source location information. If you do not want the source location
//!    information to appear in your binary file, you may prefer not to enable
//!    it.
//!
//!  - `flexible-string` improves the performance of formatting records, however
//!    contains unsafe code. For more details, see the documentation of
//!    [`StringBuf`].
//!
//!  - `log` see [Compatible with log crate](#compatible-with-log-crate) above.
//!
//! # Significant differences from C++ spdlog
//!
//! The significant differences between `spdlog-rs` and C++ `spdlog`[^1]:
//!  - `spdlog-rs` does not have `registry`[^2]. You don't need to register for
//!    loggers.
//!
//!  - `spdlog-rs` does not have `backtrace`[^2].
//!
//!  - `spdlog-rs` currently does not have `pattern_formatter`. The solution for
//!    custom formatting log messages is to implement your own [`Formatter`]
//!    structure and then call [`Sink::set_formatter`].
//!
//!  - In `spdlog-rs`, [`LevelFilter`] is a more flexible and readable enum with
//!    logical conditions.
//!
//!  - In `spdlog-rs`, there is no "_st" sinks, all sinks are "_mt".
//!
//!  - `daily_file_sink` and `hourly_file_sink` in C++ `spdlog` are merged into
//!    [`RotatingFileSink`] in `spdlog-rs`. They correspond to rotation policies
//!    [`RotationPolicy::Daily`] and [`RotationPolicy::Hourly`].
//!
//!  - Some sinks in C++ `spdlog` are not yet implemented in `spdlog-rs`. (Yes,
//!    PRs are welcome)
//!
//!  - ...
//!
//! [^1]: At the time of writing this section, the latest version of C++ `spdlog` is v1.9.2.
//!
//! [^2]: C++ `spdlog` is also planned to remove it in v2.x.
//!
//! [spdlog]: https://github.com/gabime/spdlog
//! [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/examples
//! [open a discussion]: https://github.com/SpriteOvO/spdlog-rs/discussions/new
//! [open an issue]: https://github.com/SpriteOvO/spdlog-rs/issues/new/choose
//! [log crate]: https://crates.io/crates/log
//! [`FullFormatter`]: crate::formatter::FullFormatter
//! [`RotatingFileSink`]: crate::sink::RotatingFileSink
//! [`Formatter`]: crate::formatter::Formatter
//! [`RotationPolicy::Daily`]: crate::sink::RotationPolicy::Daily
//! [`RotationPolicy::Hourly`]: crate::sink::RotationPolicy::Hourly

// Credits: https://blog.wnut.pw/2020/03/24/documentation-and-unstable-rustdoc-features/
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![warn(missing_docs)]

mod env_level;
mod error;
pub mod formatter;
mod level;
#[cfg(feature = "log")]
mod log_crate_proxy;
mod log_macros;
mod logger;
mod periodic_worker;
mod record;
pub mod sink;
mod source_location;
#[doc(hidden)]
pub mod string_buf;
pub mod terminal_style;
#[cfg(test)]
mod test_utils;
mod utils;

pub use env_level::EnvLevelError;
pub use error::*;
pub use level::*;
#[cfg(feature = "log")]
pub use log_crate_proxy::LogCrateProxy;
pub use logger::*;
pub use record::*;
pub use source_location::*;
pub use string_buf::StringBuf;

/// Contains all log macros and common types.
pub mod prelude {
    pub use super::{critical, debug, error, info, log, trace, warn};
    pub use super::{Level, LevelFilter, Logger, LoggerBuilder};
}

use std::{result::Result as StdResult, sync::Arc};

use arc_swap::ArcSwap;
use cfg_if::cfg_if;
use once_cell::sync::Lazy;

use sink::{
    Sink, {StdStream, StdStreamSink},
};
use terminal_style::StyleMode;

/// The statically resolved log level filter.
///
/// See the crate level documentation for information on how to configure this.
///
/// This value is checked by the log macros, but not by [`Logger`]s and
/// [`Sink`]s. Code that manually calls functions on these should compare the
/// level against this value.
///
/// [`Logger`]: crate::logger::Logger
/// [`Sink`]: crate::sink::Sink
pub const STATIC_LEVEL_FILTER: LevelFilter = STATIC_LEVEL_FILTER_INNER;

cfg_if! {
    if #[cfg(all(not(debug_assertions), feature = "release-level-off"))] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::Off;
    } else if #[cfg(all(not(debug_assertions), feature = "release-level-critical"))] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Critical);
    } else if #[cfg(all(not(debug_assertions), feature = "release-level-error"))] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Error);
    } else if #[cfg(all(not(debug_assertions), feature = "release-level-warn"))] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Warn);
    } else if #[cfg(all(not(debug_assertions), feature = "release-level-info"))] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Info);
    } else if #[cfg(all(not(debug_assertions), feature = "release-level-debug"))] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Debug);
    } else if #[cfg(all(not(debug_assertions), feature = "release-level-trace"))] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Trace);
    } else if #[cfg(feature = "level-off")] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::Off;
    } else if #[cfg(feature = "level-critical")] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Critical);
    } else if #[cfg(feature = "level-error")] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Error);
    } else if #[cfg(feature = "level-warn")] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Warn);
    } else if #[cfg(feature = "level-info")] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Info);
    } else if #[cfg(feature = "level-debug")] {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Debug);
    } else {
        const STATIC_LEVEL_FILTER_INNER: LevelFilter = LevelFilter::MoreSevereEqual(Level::Trace);
    }
}

#[cfg(not(windows))]
pub(crate) const EOL: &str = "\n";
#[cfg(windows)]
pub(crate) const EOL: &str = "\r\n";

static DEFAULT_LOGGER: Lazy<ArcSwap<Logger>> = Lazy::new(|| {
    let stdout = StdStreamSink::new(StdStream::Stdout, StyleMode::Auto);
    stdout.set_level_filter(LevelFilter::MoreVerbose(Level::Warn));

    let stderr = StdStreamSink::new(StdStream::Stderr, StyleMode::Auto);
    stderr.set_level_filter(LevelFilter::MoreSevereEqual(Level::Warn));

    let sinks: [Arc<dyn Sink>; 2] = [Arc::new(stdout), Arc::new(stderr)];

    ArcSwap::from_pointee(Logger::builder().sinks(sinks).build_default())
});

/// Returns an [`Arc`] default logger.
///
/// Default logger contains two [`StdStreamSink`]s, writing logs on `info` level
/// and more verbose levels to `stdout`, and writing logs on `warn` level and
/// more severe levels to `stderr`.
///
/// # Examples
///
/// ```
/// # use std::sync::Arc;
/// use spdlog::prelude::*;
///
/// let default_logger: Arc<Logger> = spdlog::default_logger();
///
/// default_logger.set_level_filter(LevelFilter::All);
///
/// info!("this log will be written to `stdout`");
/// debug!("this log will be written to `stdout`");
/// trace!("this log will be written to `stdout`");
///
/// warn!("this log will be written to `stderr`");
/// error!("this log will be written to `stderr`");
/// critical!("this log will be written to `stderr`");
/// ```
pub fn default_logger() -> Arc<Logger> {
    DEFAULT_LOGGER.load().clone()
}

/// Sets the given logger as the default logger, and returns the old default
/// logger.
///
/// # Examples
///
/// ```
/// # use std::sync::Arc;
/// use spdlog::prelude::*;
///
/// # let new_logger = spdlog::default_logger();
/// let old_logger: Arc<Logger> = spdlog::swap_default_logger(new_logger);
///
/// info!("this log will be handled by `new_logger`");
/// info!(logger: old_logger, "this log will be handled by `old_logger`");
/// ```
pub fn swap_default_logger(logger: Arc<Logger>) -> Arc<Logger> {
    DEFAULT_LOGGER.swap(logger)
}

/// Sets the given logger as the default logger.
///
/// # Examples
///
/// ```
/// # use std::sync::Arc;
/// use spdlog::prelude::*;
///
/// # let new_logger = spdlog::default_logger();
/// spdlog::set_default_logger(new_logger);
///
/// info!("this log will be handled by `new_logger`");
/// ```
pub fn set_default_logger(logger: Arc<Logger>) {
    swap_default_logger(logger);
}

/// Initialize environment variable level filters.
///
/// Returns whether the level in the environment variable was applied if there
/// are no errors.
///
/// The default level filter of loggers built after calling this function will
/// be configured based on the value of environment variable `SPDLOG_RS_LEVEL`.
///
/// Users should call this function early, the level filter of loggers built
/// before calling this function will not be configured by environment variable.
///
/// Format of the environment variable value:
///
/// - `all`
///
///   Specifies the level filter of the default logger as `LevelFilter::All`.
///
/// - `=trace`
///
///   Specifies the level filter of unnamed loggers as
/// `LevelFilter::MoreSevereEqual(Level::Trace)`.
///
/// - `example=off`
///
///   Specifies the level filter of loggers with name "example" as
/// `LevelFilter::Off`.
///
/// - `*=error`
///
///   Specifies the level filter of all loggers (except the default logger) as
/// `LevelFilter::MoreSevereEqual(Level::Error)` (respect the above rules if
/// they are matched).
///
/// The level filter is not case-sensitive, and these rules are combinable,
/// separated by commas. For example, these are legal:
///
/// - `ALL,*=ALL`
///
///   Specifies the level filter of all loggers as `LevelFilter::All`.
///
/// - `off,*=ERROR`
///
///   Specifies the level filter of the default logger as `LevelFilter::Off`,
/// the rest of loggers as `LevelFilter::MoreSevereEqual(Level::Error)`.
///
/// - `gui=warn,network=trace`
///
///   Specifies the level filter of loggers with name "gui" as
/// `LevelFilter::MoreSevereEqual(Level::Warn)`, loggers with name "network" as
/// `LevelFilter::MoreSevereEqual(Level::Trace)`.
///
/// However, the same rule cannot be specified more than once.
///
/// # Examples
///
/// - Environment variable `SPDLOG_RS_LEVEL` is not present:
///
///   ```
///   use spdlog::prelude::*;
///
///   # fn main() -> Result<(), spdlog::EnvLevelError> {
///   assert_eq!(spdlog::init_env_level()?, false);
///
///   assert_eq!(
///       spdlog::default_logger().level_filter(),
///       LevelFilter::MoreSevereEqual(Level::Info) // default level filter
///   );
///   assert_eq!(
///       Logger::builder().build().level_filter(), // unnamed logger
///       LevelFilter::MoreSevereEqual(Level::Info) // default level filter
///   );
///   assert_eq!(
///       Logger::builder().name("gui").build().level_filter(),
///       LevelFilter::MoreSevereEqual(Level::Info) // default level filter
///   );
///   assert_eq!(
///       Logger::builder().name("network").build().level_filter(),
///       LevelFilter::MoreSevereEqual(Level::Info) // default level filter
///   );
///   # Ok(()) }
///   ```
///
/// - `SPDLOG_RS_LEVEL="TRACE,network=Warn,*=error"`:
///
///   ```
///   use spdlog::prelude::*;
///
///   # fn main() -> Result<(), spdlog::EnvLevelError> {
///   # std::env::set_var("SPDLOG_RS_LEVEL", "TRACE,network=Warn,*=error");
///   assert_eq!(spdlog::init_env_level()?, true);
///
///   assert_eq!(
///       spdlog::default_logger().level_filter(),
///       LevelFilter::MoreSevereEqual(Level::Trace)
///   );
///   assert_eq!(
///       Logger::builder().build().level_filter(), // unnamed logger
///       LevelFilter::MoreSevereEqual(Level::Error)
///   );
///   assert_eq!(
///       Logger::builder().name("gui").build().level_filter(),
///       LevelFilter::MoreSevereEqual(Level::Error)
///   );
///   assert_eq!(
///       Logger::builder().name("network").build().level_filter(),
///       LevelFilter::MoreSevereEqual(Level::Warn)
///   );
///   # Ok(()) }
///   ```
///
/// - `SPDLOG_RS_LEVEL="network=Warn,network=Warn` will fail, as the same rule
///   is specified multiple times.
///
///   ```
///   # std::env::set_var("SPDLOG_RS_LEVEL", "network=Warn,network=Warn");
///   let result = spdlog::init_env_level();
///   # // TODO: commented out since `assert_matches` is currently unstable.
///   # //       change to use `assert_matches` when it is stable.
///   # // assert_matches!(result, Err(spdlog::EnvLevelError::ParseEnvVar(_)));
///   if let Err(spdlog::EnvLevelError::ParseEnvVar(_)) = result {
///       // expected
///   } else {
///       // unexpected
///       assert!(false);
///   }
///   ```
pub fn init_env_level() -> StdResult<bool, EnvLevelError> {
    env_level::from_env("SPDLOG_RS_LEVEL")
}

/// Initialize log crate proxy.
///
/// This function calls [`log::set_logger`] to set up a [`LogCrateProxy`] and
/// all logs from log crate will be forwarded to `spdlog-rs`'s logger.
///
/// Users should call this function only once. Get the proxy to configure by
/// calling [`log_crate_proxy()`].
///
/// Note that the `log` crate uses a different log level filter and by default
/// it rejects all log messages. To log messages via the `log` crate, you have
/// to call [`log::set_max_level`] manually before logging. For more
/// information, please read the documentation of [`log::set_max_level`].
///
/// For more details, please read documentation of [`log::set_logger`] and
/// [`LogCrateProxy`].
#[cfg(feature = "log")]
pub fn init_log_crate_proxy() -> StdResult<(), log::SetLoggerError> {
    log::set_logger(log_crate_proxy())
}

/// Returns a [`LogCrateProxy`].
#[cfg(feature = "log")]
pub fn log_crate_proxy() -> &'static LogCrateProxy {
    static PROXY: Lazy<LogCrateProxy> = Lazy::new(LogCrateProxy::new);
    &PROXY
}

fn default_error_handler(from: impl AsRef<str>, error: Error) {
    let date = chrono::Local::now()
        .format("%Y-%m-%d %H:%M:%S.%3f")
        .to_string();

    eprintln!(
        "[*** SPDLOG-RS UNHANDLED ERROR ***] [{}] [{}] {}",
        date,
        from.as_ref(),
        error
    );
}

// Used at log macros
#[doc(hidden)]
pub fn __log(
    logger: &Logger,
    level: Level,
    srcloc: Option<SourceLocation>,
    fmt_args: std::fmt::Arguments,
) {
    // use `Cow` to avoid allocation as much as we can
    let payload: std::borrow::Cow<str> = match fmt_args.as_str() {
        Some(literal_str) => literal_str.into(), // no format arguments, so it is a `&'static str`
        None => fmt_args.to_string().into(),
    };

    let mut builder = Record::builder(level, payload).source_location(srcloc);
    if let Some(logger_name) = logger.name() {
        builder = builder.logger_name(logger_name);
    }
    logger.log(&builder.build());
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_utils::*;

    #[test]
    fn test_default_logger() {
        let test_sink = Arc::new(CounterSink::new());

        let test_logger = Arc::new(test_logger_builder().sink(test_sink.clone()).build());
        let empty_logger = Arc::new(Logger::builder().build());

        set_default_logger(empty_logger.clone());
        info!("hello");
        error!("world");

        set_default_logger(test_logger);
        warn!("hello");
        error!("rust");

        set_default_logger(empty_logger);
        info!("hello");
        error!("spdlog");

        assert_eq!(test_sink.log_count(), 2);
        assert_eq!(
            test_sink.payloads(),
            vec!["hello".to_string(), "rust".to_string()]
        );
    }
}
