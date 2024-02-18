//! copy-paste of simple logger; no deps work with doctests for some reason.
//! feature-blocked structs and functions are removed.
//!
//! A logger that prints all messages with a simple, readable output format.
//!
//! Optional features include timestamps, colored output and logging to stderr.
//!
//! ```ignore
//! simple_logger::SimpleLogger::new().env().init().unwrap();
//!
//! log::warn!("This is an example message.");
//! ```
//!
//! Some shortcuts are available for common use cases.
//!
//! Just initialize logging without any configuration:
//!
//! ```ignore
//! simple_logger::init().unwrap();
//! ```
//!
//! Set the log level from the `RUST_LOG` environment variable:
//!
//! ```ignore
//! simple_logger::init_with_env().unwrap();
//! ```
//!
//! Hardcode a default log level:
//!
//! ```ignore
//! simple_logger::init_with_level(log::Level::Warn).unwrap();
//! ```

use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
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

    /// Set the 'default' log level.
    ///
    /// You can override the default level for specific modules and their sub-modules using [`with_module_level`]
    ///
    /// This must be called before [`env`]. If called after [`env`], it will override the value loaded from the environment.
    ///
    /// [`env`]: #method.env
    /// [`with_module_level`]: #method.with_module_level
    #[must_use = "You must call init() to begin logging"]
    pub const fn with_level(mut self, level: LevelFilter) -> Self {
        self.default_level = level;
        self
    }

    /// Override the log level for some specific modules.
    ///
    /// This sets the log level of a specific module and all its sub-modules.
    /// When both the level for a parent module as well as a child module are set,
    /// the more specific value is taken. If the log level for the same module is
    /// specified twice, the resulting log level is implementation defined.
    ///
    /// # Examples
    ///
    /// Silence an overly verbose crate:
    ///
    /// ```ignore
    /// use log::LevelFilter;
    /// use simple_logger::SimpleLogger;
    ///
    /// SimpleLogger::new()
    ///     .with_module_level("chatty_dependency", LevelFilter::Warn)
    ///     .init()
    ///     .unwrap();
    /// ```
    ///
    /// Disable logging for all dependencies:
    ///
    /// ```ignore
    /// use log::LevelFilter;
    /// use simple_logger::SimpleLogger;
    ///
    /// SimpleLogger::new()
    ///     .with_level(LevelFilter::Off)
    ///     .with_module_level("my_crate", LevelFilter::Info)
    ///     .init()
    ///     .unwrap();
    /// ```
    //
    // This method *must* sort `module_levels` for the [`enabled`](#method.enabled) method to work correctly.
    #[must_use = "You must call init() to begin logging"]
    pub fn with_module_level(mut self, target: &str, level: LevelFilter) -> Self {
        self.module_levels.push((target.to_string(), level));
        self.module_levels
            .sort_by_key(|(name, _level)| name.len().wrapping_neg());
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

/// Initialise the logger with its default configuration.
///
/// Log messages will not be filtered.
/// The `RUST_LOG` environment variable is not used.
pub fn init() -> Result<(), SetLoggerError> {
    Simple::new().init()
}

/// Initialise the logger with the `RUST_LOG` environment variable.
///
/// Log messages will be filtered based on the `RUST_LOG` environment variable.
pub fn init_with_env() -> Result<(), SetLoggerError> {
    Simple::new().env().init()
}

/// Initialise the logger with a specific log level.
///
/// Log messages below the given [`Level`] will be filtered.
/// The `RUST_LOG` environment variable is not used.
pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    Simple::new().with_level(level.to_level_filter()).init()
}
