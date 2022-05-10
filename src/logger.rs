use env_logger::{Builder, Env, Target};
#[cfg(not(debug_assertions))]
use std::{fs::File, io::BufWriter};

pub fn init_logger(level: log::LevelFilter) {
    let env = Env::default().default_filter_or("wgpu_core=warn,wgpu_hal=warn,naga=warn,debug");
    let mut builder = Builder::from_env(env);
    builder.filter_level(level);
    builder.target(Target::Stdout);
    #[cfg(not(debug_assertions))]
    match get_log_file_writer() {
        Ok(writer) => {
            builder.target(Target::Pipe(Box::new(writer)));
        }
        _ => {}
    }
    builder.init();
    log::info!("successfuly configured global logge with level {level}");
}

#[cfg(not(debug_assertions))]
fn get_log_file_writer() -> anyhow::Result<BufWriter<File>> {
    const LOG_FILE: &str = "/tmp/gbmu.log";
    const BUFFER_CAPS: usize = 32_768;

    let file = File::create(LOG_FILE).map_err(anyhow::Error::from)?;

    Ok(BufWriter::with_capacity(BUFFER_CAPS, file))
}
