use std::{collections::HashMap, sync::Arc};

use flume::Sender;
use log::{LevelFilter, Log};
use pyo3::{types::PyModule, Py, PyObject, PyResult, Python};

struct LoggerConfig {
    /// Filter used as a fallback if none of the `filters` match.
    top_filter: LevelFilter,

    /// Mapping of filters to modules.
    ///
    /// The most specific one will be used, falling back to `top_filter` if none matches.
    /// Stored as full paths, with `::` separaters (eg. before converting them from Rust to Python).
    filters: HashMap<String, LevelFilter>,

    /// The imported Python `logging` module.
    logging: Py<PyModule>,

    cache: Arc<CacheNode>,
}

impl LoggerConfig {
    fn new(py: Python<'_>) -> PyResult<Self> {
        let logging = py.import("logging")?;
        Ok(Self {
            top_filter: LevelFilter::Debug,
            filters: HashMap::default(),
            logging: logging.into(),
            cache: Arc::default(),
        })
    }

    fn install(self) -> Result<(), log::SetLoggerError> {
        let level = std::cmp::max(
            self.top_filter,
            self.filters
                .values()
                .max()
                .copied()
                .unwrap_or(LevelFilter::Off),
        );
        let config = Arc::new(SharedConfig { cache: self.cache });
        let send_log_channel = BackgroundLogger::new(config.clone()).spawn();
        let foreground_logger = ForegroundLogger::new(send_log_channel, config);
        log::set_boxed_logger(Box::new(foreground_logger))?;
        log::set_max_level(level);
        Ok(())
    }
}

#[derive(Default)]
struct CacheNode {
    local: Option<CacheEntry>,
    children: HashMap<String, Arc<CacheNode>>,
}

struct CacheEntry {
    filter: LevelFilter,
    logger: PyObject,
}

struct BackgroundLogger {
    config: Arc<SharedConfig>,
}

impl BackgroundLogger {
    fn new(config: Arc<SharedConfig>) -> Self {
        Self { config }
    }

    fn spawn(self) -> Sender<LogRecord> {
        let (sender, receiver) = flume::bounded(5);

        std::thread::spawn(move || {
            let BackgroundLogger { config } = self;

            for record in receiver.into_iter() {}
        });

        sender
    }
}

struct LogRecord {
    level: log::Level,
}

impl From<&log::Record<'_>> for LogRecord {
    fn from(value: &log::Record) -> Self {
        Self {
            level: value.level(),
        }
    }
}

struct ForegroundLogger {
    sender: Sender<LogRecord>,
    config: Arc<SharedConfig>,
}

impl ForegroundLogger {
    fn new(sender: Sender<LogRecord>, config: Arc<SharedConfig>) -> Self {
        Self { sender, config }
    }
}

impl Log for ForegroundLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        todo!()
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            self.sender.send(record.into()).expect("Cannot send log");
        }
    }

    fn flush(&self) {}
}

struct SharedConfig {
    cache: Arc<CacheNode>,
}

impl SharedConfig {
    /// Lockup for the associated [CacheNode] for the given `target`.
    fn lookup(&self, target: &str) -> Option<Arc<CacheNode>> {
        let mut node = &self.cache;
        for segment in target.split("::") {
            match node.children.get(segment) {
                Some(child) => node = child,
                None => return None,
            }
        }

        Some(node.clone())
    }
}
