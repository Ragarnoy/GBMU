use clap::Clap;
use std::{fs::File, io::Read, iter::Peekable};

use gb_roms::OpcodeGenerator;

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "fbenneto")]
struct DisasOpt {
    #[clap(required = true)]
    files: Vec<String>,
}

fn disas_file(name: &str) {
    println!("current file: \"{}\"", name);
    let file = File::open(name).expect("cannot open file");

    let mut it = file.bytes().enumerate().map(|(pos, v)| (pos, v.unwrap()));
    test(&mut it.by_ref().take(0x100 + 4).peekable());
    let it = it.skip(0x50 - 4); // skip header - first 4 bytes
    test(&mut it.peekable());
}

fn test(it: &mut Peekable<impl Iterator<Item = (usize, u8)>>) {
    while let Some((pos, v)) = it.peek() {
        let current_pos: usize = *pos;
        let current_opcode: u8 = *v;
        let mut gen = OpcodeGenerator::from(it.map(|(_, v)| v));
        match gen.next().expect("expected opcode") {
            Ok((op, bytes)) => println!("{:#08x} ({:2x}): {:10}: {:?}", current_pos, current_opcode, op, bytes),
            Err(e) => eprintln!(
                "position={:x}, opcode={:x}, error={:x?}",
                current_pos, current_opcode, e
            ),
        }
    }
}

fn main() {
    let opts: DisasOpt = DisasOpt::parse();
    opts.files.iter().for_each(|f| disas_file(f));
}
