use cpal::traits::{DeviceTrait, HostTrait};

fn main() {
    let host = cpal::default_host();
    let devices = host.devices().expect("cannot get devices");

    for (id, device) in devices.into_iter().enumerate() {
        let name = device.name();
        let output_configs = device.supported_output_configs();

        println!("┌ Device #{id}");
        match name {
            Ok(name) => println!("│ name: {name}"),
            Err(e) => println!("│ name: ERROR: {e}"),
        }
        match output_configs {
            Ok(configs) => {
                for (config_id, config) in configs.into_iter().enumerate() {
                    let supported_config = config.with_max_sample_rate();
                    let sample_format = supported_config.sample_format();
                    let sample_rate = supported_config.sample_rate();
                    let buffer_size = supported_config.buffer_size();
                    let channels = supported_config.channels();

                    println!("│ config_{config_id}:");
                    println!("│   sample_format: {sample_format:?}");
                    println!("│   stream_rate: {sample_rate:?}");
                    println!("│   buffer_size: {buffer_size:?}");
                    println!("│   channels: {channels}");
                }
            }
            Err(e) => println!("│ configs: NO CONFIG: {e}"),
        }
        println!("└")
    }
}
