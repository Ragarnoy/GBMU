use clap::Clap;

use gb_roms::opcode::OpcodeBits;

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "fbenneto")]
struct DecomposeOpts {
	#[clap(required = true, validator=parse_value)]
	opcodes: Vec<String>,
}

fn parse_value(arg: &str) -> Result<u8, std::num::ParseIntError> {
	if arg.starts_with("0x") {
		u8::from_str_radix(&arg[2..], 16)
	} else if arg.starts_with("0o") {
		u8::from_str_radix(&arg[2..], 8)
	} else if arg.starts_with("0b") {
		u8::from_str_radix(&arg[2..], 2)
	} else {
		arg.parse::<u8>()
	}
}

fn main() {
	let opts = DecomposeOpts::parse();
	println!("{:x?}", opts);
	for v in &opts.opcodes {
		let n = parse_value(v).unwrap();
		let bits = OpcodeBits::from_bytes([n]);
		println!("n   : {:}", n);
		println!("n   : {:#2x}", n);
		println!("n   : {:#3o}", n);
		println!("n   : {:#8b}", n);
		println!("bits: {:x?}", bits);
	}
}
