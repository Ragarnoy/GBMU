use log::{set_boxed_logger, set_max_level, LevelFilter};
#[cfg(not(debug_assertions))]
use simplelog::WriteLogger;
use simplelog::{Config, TermLogger};
#[cfg(not(debug_assertions))]
use std::{fs::File, io::BufWriter};

#[cfg(debug_assertions)]
pub fn init_logger(level: log::LevelFilter) {
    let logger = terminal_logger(level, Config::default());
    set_max_level(level);
    set_boxed_logger(logger).expect("cannot set logger");
    log::info!(
        "successfuly configured terminal to log entry with level {}",
        level
    )
}

#[cfg(not(debug_assertions))]
pub fn init_logger(level: log::LevelFilter) {
    let config: Config = Config::default();

    set_max_level(level);
    if let Err(e) = init_file_logger(level, config.clone()) {
        set_boxed_logger(terminal_logger(level, config)).expect("cannot set any logger");
        log::error!("failed to set file logger, fallback to terminal: {}", e);
    } else {
        log::info!(
            "successfuly configured file logger to log entry with level {}",
            level
        );
    }
}

#[cfg(not(debug_assertions))]
fn init_file_logger(level: log::LevelFilter, config: Config) -> anyhow::Result<()> {
    const LOG_FILE: &str = "/tmp/gbmu.log";
    const TRACE_LOG_FILE: &str = "/tmp/gbmu-trace.log";
    const LEVEL_BEFORE_BUFFERING: LevelFilter = LevelFilter::Info;

    if level > LEVEL_BEFORE_BUFFERING {
        use simplelog::CombinedLogger;

        CombinedLogger::init(vec![
            file_logger(LOG_FILE, LEVEL_BEFORE_BUFFERING, config.clone())?,
            buffered_file_logger(TRACE_LOG_FILE, level, config)?,
        ])?
    } else {
        set_boxed_logger(file_logger(LOG_FILE, level, config)?)?
    }
    Ok(())
}

#[cfg(not(debug_assertions))]
fn buffered_file_logger(
    filename: &str,
    level: LevelFilter,
    config: Config,
) -> anyhow::Result<Box<WriteLogger<BufWriter<File>>>> {
    const BUFFER_CAPS: usize = 32_768;
    let file = File::create(filename)?;

    Ok(WriteLogger::new(
        level,
        config,
        BufWriter::with_capacity(BUFFER_CAPS, file),
    ))
}

fn terminal_logger(level: LevelFilter, config: Config) -> Box<TermLogger> {
    use simplelog::{ColorChoice, TerminalMode};

    TermLogger::new(level, config, TerminalMode::Mixed, ColorChoice::Auto)
}

#[cfg(not(debug_assertions))]
fn file_logger(
    filename: &str,
    level: LevelFilter,
    config: Config,
) -> anyhow::Result<Box<WriteLogger<File>>> {
    let file = File::create(filename)?;

    Ok(WriteLogger::new(level, config, file))
}
