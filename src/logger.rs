use crate::config::LoggingSettings;
use log::{LevelFilter, SetLoggerError};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
    sync::{Mutex, OnceLock},
};

static LOGGER: OnceLock<SimpleLogger> = OnceLock::new();

struct SimpleLogger {
    level: LevelFilter,
    file: Option<Mutex<File>>,
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let msg = format!("[{}] {}\n", record.level(), record.args());
            eprint!("{msg}");
            if let Some(file) = &self.file
                && let Ok(mut f) = file.lock()
            {
                let _ = f.write_all(msg.as_bytes());
            }
        }
    }

    fn flush(&self) {}
}

pub fn init_logger(settings: &LoggingSettings) -> Result<(), SetLoggerError> {
    let level = match settings.level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    let log_path = settings.file.as_ref().map_or_else(
        || Path::new("log/promptly.log"),
        |file_str| Path::new(file_str),
    );

    let log_path = Path::new("log").join(
        log_path
            .file_name()
            .unwrap_or_else(|| std::ffi::OsStr::new("promptly.log")),
    );

    if let Some(parent) = log_path.parent()
        && !parent.exists()
    {
        fs::create_dir_all(parent).expect("Failed to create log directory");
    }

    let file = Some(Mutex::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .expect("Failed to open log file"),
    ));

    let logger = SimpleLogger { level, file };
    let logger_ref = LOGGER.get_or_init(|| logger);

    log::set_logger(logger_ref)?;
    log::set_max_level(level);
    log::info!("Logging initialized at {}", log_path.display());

    Ok(())
}
