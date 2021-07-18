use clap::Clap;
use std::{fs::File, io::Read};

use gb_roms::OpcodeGenerator;

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "fbenneto")]
struct DisasOpt {
	#[clap(required = true)]
	files: Vec<String>,
}

fn disas_file(name: &String) {
	println!("current file: \"{}\"", name);
	let file = File::open(name).expect("cannot open file");

	test(file.bytes().map(|v| {
		println!("readed: {:x?}", v);
		v.unwrap()
	}));
}

fn test(it: impl Iterator<Item = u8>) {
	let mut gen = OpcodeGenerator::from(it);
	while let Some(op) = gen.next() {
		println!("op: {}", op.expect("error while reading opcode"));
	}
}

fn main() {
	let opts: DisasOpt = DisasOpt::parse();
	opts.files.iter().for_each(disas_file);
}
