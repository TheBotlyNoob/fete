//! copy-paste and stripped down of `simple_logger`; no deps work with doctests for some reason.
//!
//! A logger that prints all messages with a simple, readable output format.
//!
//! ```ignore
//! simple_logger::SimpleLogger::new().env().init().unwrap();
//!
//! log::warn!("This is an example message.");
//! ```
//!
//! Some shortcuts are available for common use cases.
//!
//! Set the log level from the `RUST_LOG` environment variable:
//!
//! ```ignore
//! simple_logger::init_with_env().unwrap();
//! ```

use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::str::FromStr;

/// Implements [`Log`] and a set of simple builder methods for configuration.
///
/// Use the various "builder" methods on this struct to configure the logger,
/// then call [`init`] to configure the [`log`] crate.
pub struct Simple {
    /// The default logging level
    default_level: LevelFilter,

    /// The specific logging level for each module
    ///
    /// This is used to override the default value for some specific modules.
    ///
    /// This must be sorted from most-specific to least-specific, so that [`enabled`](#method.enabled) can scan the
    /// vector for the first match to give us the desired log level for a module.
    module_levels: Vec<(String, LevelFilter)>,
}

impl Simple {
    /// Initializes the global logger with a `SimpleLogger` instance with
    /// default log level set to `Level::Trace`.
    ///
    /// ```ignore
    /// use simple_logger::SimpleLogger;
    /// SimpleLogger::new().env().init().unwrap();
    /// log::warn!("This is an example message.");
    /// ```
    ///
    /// [`init`]: #method.init
    #[must_use = "You must call init() to begin logging"]
    pub const fn new() -> Self {
        Self {
            default_level: LevelFilter::Trace,
            module_levels: Vec::new(),
        }
    }

    /// Enables the user to choose log level by setting `RUST_LOG=<level>`
    /// environment variable. This will use the default level set by
    /// [`with_level`] if `RUST_LOG` is not set or can't be parsed as a
    /// standard log level.
    ///
    /// This must be called after [`with_level`]. If called before
    /// [`with_level`], it will have no effect.
    ///
    /// [`with_level`]: #method.with_level
    #[must_use = "You must call init() to begin logging"]
    pub fn env(mut self) -> Self {
        self.default_level = std::env::var("RUST_LOG")
            .ok()
            .as_deref()
            .map(log::LevelFilter::from_str)
            .and_then(Result::ok)
            .unwrap_or(self.default_level);

        self
    }

    /// Configure the logger
    pub fn max_level(&self) -> LevelFilter {
        let max_level = self
            .module_levels
            .iter()
            .map(|(_name, level)| level)
            .copied()
            .max();
        max_level.map_or(self.default_level, |lvl| lvl.max(self.default_level))
    }

    /// 'Init' the actual logger and instantiate it,
    /// this method MUST be called in order for the logger to be effective.
    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_max_level(self.max_level());
        log::set_logger(Box::leak(Box::new(self)))
    }
}

impl Default for Simple {
    /// See [`Simple::new`](struct.SimpleLogger.html#method.new)
    fn default() -> Self {
        Self::new()
    }
}

impl Log for Simple {
    fn enabled(&self, metadata: &Metadata) -> bool {
        &metadata.level().to_level_filter()
            <= self
                .module_levels
                .iter()
                /* At this point the Vec is already sorted so that we can simply take
                 * the first match
                 */
                .find(|(name, _level)| metadata.target().starts_with(name))
                .map_or(&self.default_level, |(_name, level)| level)
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level_string = format!("{:<5}", record.level().to_string());

            let target = if record.target().is_empty() {
                record.module_path().unwrap_or_default()
            } else {
                record.target()
            };

            let message = format!("{level_string} [{target}] {}", record.args());

            eprintln!("{message}");
        }
    }

    fn flush(&self) {}
}

/// Initialise the logger with the `RUST_LOG` environment variable.
///
/// Log messages will be filtered based on the `RUST_LOG` environment variable.
pub fn init_with_env() -> Result<(), SetLoggerError> {
    Simple::new().env().init()
}
