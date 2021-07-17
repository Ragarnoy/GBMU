use clap::Clap;
use std::{
	convert::TryFrom,
	fs::File,
	io::{Read, Seek, SeekFrom},
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
	file.seek(SeekFrom::Start(0x100))
		.expect("failed to seek to header");
	{
		let mut header_chunk = [0_u8; 80];
		file.read(&mut header_chunk)
			.expect("error while reading header chunk");
		let raw_header = RawHeader::from(&header_chunk);
		let header = Header::try_from(raw_header).expect("cannot convert raw header to header");
		println!("header      : {:?}", header);
	}
}

fn main() {
	let opts: CliOpts = CliOpts::parse();
	opts.files.iter().for_each(get_gb_header_from_file);
}
