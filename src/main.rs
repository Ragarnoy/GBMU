mod config;

use config::Config;
use pixels::Error;

fn main() -> Result<(), Error> {
    #[cfg(feature = "cgb")]
    let mut opts: Config = Config::parse();
    #[cfg(not(feature = "cgb"))]
    let opts: Config = Config::parse();
    #[cfg(feature = "time_frame")]
    let mut time_frame_stat = time_frame::TimeStat::default();
    #[cfg(any(feature = "time_frame", feature = "debug_fps"))]
    let mut render_time_frame = time_frame::TimeStat::default();
    let frame_duration_target = Duration::from_nanos(10_000_000_000 / TARGET_FPS_X10);
    init_logger(opts.log_level);

    Ok(())
}

fn init(config: &Config) {}
