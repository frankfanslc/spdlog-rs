//! Provides error types.

use std::{fmt, io, result};

use thiserror::Error;

/// The error type of this crate.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// The variant returned by [`Formatter`]s when an error occurs in
    /// formatting a record.
    ///
    /// [`Formatter`]: crate::formatter::Formatter
    #[error("format record error: {0}")]
    FormatRecord(fmt::Error),

    /// The variant returned by [`Sink`]s when an error occurs in writing a
    /// record to the target.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[error("write record error: {0}")]
    WriteRecord(io::Error),

    /// The variant returned by [`Sink`]s when an error occurs in flushing the
    /// buffer.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[error("flush buffer error: {0}")]
    FlushBuffer(io::Error),

    /// The variant returned by [`Sink`]s when an error occurs in creating a
    /// directory.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[error("create directory error: {0}")]
    CreateDirectory(io::Error),

    /// The variant returned by [`Sink`]s when an error occurs in opening a
    /// file.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[error("open file error: {0}")]
    OpenFile(io::Error),

    /// The variant returned by [`Sink`]s when an error occurs in querying the
    /// metadata of a file.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[error("query file metadata error: {0}")]
    QueryFileMetadata(io::Error),

    /// The variant returned by [`Sink`]s when an error occurs in renaming a
    /// file.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[error("rename file error: {0}")]
    RenameFile(io::Error),

    /// The variant returned by [`Sink`]s when an error occurs in removing a
    /// file.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[error("remove file error: {0}")]
    RemoveFile(io::Error),

    /// The variant returned by [`from_str`] when the string doesn't match any
    /// of the log levels.
    ///
    /// [`from_str`]: std::str::FromStr::from_str
    #[error("attempted to convert a string that doesn't match an existing log level: {0}")]
    ParseLevel(String),
}

/// The result type of this crate.
pub type Result<T> = result::Result<T, Error>;

/// The error handler function type.
pub type ErrorHandler = fn(Error);
