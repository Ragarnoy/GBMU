#[cfg(debug_assertions)]
pub fn init_logger() {
    use log::LevelFilter;
    use simplelog::Config;

    setup_terminal_logger(LevelFilter::Debug, Config::default());
}

#[cfg(not(debug_assertions))]
pub fn init_logger() {
    use log::LevelFilter;
    use simplelog::{Config, WriteLogger};
    use std::fs::File;

    const LEVEL_FILTER: LevelFilter = LevelFilter::Warn;
    const LOG_FILE: &'static str = "/tmp/gbmu.log";
    let config: Config = Config::default();
    let file_res = File::create(LOG_FILE);

    if let Ok(file) = file_res {
        let write_logger_res = WriteLogger::init(LEVEL_FILTER, config.clone(), file);
        if write_logger_res.is_ok() {
            return;
        } else {
            setup_terminal_logger(LEVEL_FILTER, config);
            log::warn!(
                "cannot setup write logger (because: {})",
                write_logger_res.unwrap_err()
            );
        }
    } else {
        setup_terminal_logger(LEVEL_FILTER, config);
        log::warn!(
            "cannot setup logging to file {} (because: {})",
            LOG_FILE,
            file_res.unwrap_err()
        );
    }
    log::warn!("fallback to terminal logger");
}

fn setup_terminal_logger(level: log::LevelFilter, config: simplelog::Config) {
    use simplelog::{ColorChoice, TermLogger, TerminalMode};

    TermLogger::init(level, config, TerminalMode::Mixed, ColorChoice::Auto)
        .expect("cannot setup terminal logger")
}
