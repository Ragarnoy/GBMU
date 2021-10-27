use clap::Clap;
use std::fs::File;

use gb_roms::Header;

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "fbenneto")]
struct CliOpts {
    #[clap(required = true)]
    files: Vec<String>,
}

fn get_gb_header_from_file(name: &str) {
    println!("current file: \"{}\"", name);
    let mut file = File::open(name).expect("cannot open file");

    let header = Header::from_file(&mut file).expect("failed to read header");
    println!("header      : {:02x?}", header);
}

fn main() {
    let opts: CliOpts = CliOpts::parse();
    opts.files.iter().for_each(|s| get_gb_header_from_file(s));
}
