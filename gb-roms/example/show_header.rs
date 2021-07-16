use clap::Clap;
use std::{
	convert::TryFrom,
	fs::File,
	io::{BufRead, BufReader, Read},
};

use gb_roms::{Header, RawHeader};

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "fbenneto")]
struct CliOpts {
	#[clap(required = true)]
	files: Vec<String>,
}

fn get_gb_header_from_file(name: &String) {
	println!("current file: \"{}\"", name);
	let mut file = File::open(name).expect("cannot open file");
	{
		let mut skipped = [0_u8; 0x100];
		file.read(&mut skipped)
			.expect("error while reading chunk before header");
	}
	{
		let mut header_chunk = [0_u8; 80];
		file.read(&mut header_chunk)
			.expect("error while reading header chunk");
		println!("header chunk: {:?}", header_chunk);

		let raw_header = RawHeader::from(&header_chunk);
		println!("raw header  : {:?}", raw_header);

		let header = Header::try_from(raw_header);
		println!("header      : {:?}", header);
	}
}

fn main() {
	let opts: CliOpts = CliOpts::parse();
	opts.files.iter().for_each(get_gb_header_from_file);
}
