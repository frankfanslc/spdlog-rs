//! Provides a string buffer type for sinks and formatters.

/// A string buffer type.
///
/// Used at [`Formatter`].
///
/// By default, it is an alias for [`String`], if feature `flexible-string` is
/// enabled, an internal type `FlexibleString` will be used.
///
/// `FlexibleString` has a fixed stack buffer of 250 bytes, and upgrades to
/// [`String`] when more space is needed. It provides APIs that are as
/// consistent as possible with [`String`], but some APIs are not yet
/// implemented or cannot be implemented.
///
/// # Warnings
///
/// `FlexibleString` can improve performance as it avoids memory allocation when
/// formatting records (most log messages do not exceed 250 bytes), however it
/// contains unsafe code.
///
/// [`Sink`]: crate::sink::Sink
/// [`Formatter`]: crate::formatter::Formatter
pub type StringBuf = StringBufInner;

use cfg_if::cfg_if;

// Users should not use the following types directly.

cfg_if! {
    if #[cfg(feature = "flexible-string")] {
        // pub for hide type alias in doc
        #[doc(hidden)]
        pub type StringBufInner = flexible_string::FlexibleString<250>;
    } else {
        // same as above
        #[doc(hidden)]
        pub type StringBufInner = String;
    }
}
