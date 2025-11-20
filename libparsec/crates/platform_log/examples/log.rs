use libparsec_platform_log::{init_logger, LogConfig};
use tracing::level_filters::LevelFilter;

fn main() {
    let log_file = std::env::temp_dir().join("test.log");
    std::fs::write(&log_file, b"").unwrap();
    println!("Will write log to {}", log_file.display());
    let config = LogConfig {
        level: Some(LevelFilter::TRACE),
        log_file: Some(log_file),
        log_console: true,
    };
    init_logger(config).unwrap();

    #[cfg(feature = "example")]
    libparsec_platform_log::test_tracing();
}
