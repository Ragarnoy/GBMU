use clap::Clap;
use std::{
	fs::File,
	io::{Read, Seek, SeekFrom},
};

use gb_roms::OpcodeGenerator;

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "fbenneto")]
struct DisasOpt {
	#[clap(required = true)]
	files: Vec<String>,

	#[clap(short, long, default_value = "0")]
	start_at: u64,
}

fn disas_file(name: &String, start: u64) {
	println!("current file: \"{}\"", name);
	let mut file = File::open(name).expect("cannot open file");

	if start != 0 {
		file.seek(SeekFrom::Start(start)).expect("cannot seek");
	}
	test(file.bytes().map(|v| {
		println!("readed: {:x?}", v);
		v.unwrap()
	}));
}

fn test(it: impl Iterator<Item = u8>) {
	let mut gen = OpcodeGenerator::from(it);
	while let Some(op) = gen.next() {
		match op {
			Ok(op) => println!("op: {}", op),
			Err(e) => eprintln!("error: {:?}", e),
		}
	}
}

fn main() {
	let opts: DisasOpt = DisasOpt::parse();
	opts.files.iter().for_each(|f| disas_file(f, opts.start_at));
}
