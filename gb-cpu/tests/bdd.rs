use async_trait::async_trait;
use cucumber::{given, then, when, World, WorldInit};
use gb_cpu::{cpu::Cpu, registers::Registers};
use std::{convert::Infallible, str::FromStr, fmt::{self, Debug}};
use futures::executor::block_on;
use gb_bus::{Bus, Error};
use gb_clock::Ticker;

#[derive(Clone,  WorldInit, Default)]
struct CpuWorld {
    cpu: Cpu,
    bus: MockBus,
}

impl Debug for CpuWorld {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CpuWorld {{ cpu: {:?}, bus: {{ ... }} }}", self.cpu)
    }
}

#[async_trait(?Send)]
impl World for CpuWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Self::Error> {
        Ok(Self::default())
    }
}

#[derive(Clone, Debug)]
struct MockBus {
    store: [u8; u16::MAX as usize],
}

impl Default for MockBus {
    fn default() -> Self {
        Self { store: [0; u16::MAX as usize] }
    }
}

impl Bus<u8> for MockBus {
    fn read(&self, address: u16) -> Result<u8, Error> {
        Ok(self.store[address as usize])
    }

    fn write(&mut self, address: u16, data: u8) -> Result<(), Error> {
        self.store[address as usize] = data;
        Ok(())
    }
}

impl Bus<u16> for MockBus {
    fn read(&self, _address: u16) -> Result<u16, Error> {
        unimplemented!();
    }

    fn write(&mut self, _address: u16, _data: u16) -> Result<(), Error> {
        unimplemented!();
    }
}

enum Reg16 {
    PC,
}

impl Reg16 {
    fn read_corresponding_regs(&self, regs: &Registers) -> u16 {
        match self {
            Reg16::PC => regs.pc
        }
    }
}

impl FromStr for Reg16 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PC" => Ok(Reg16::PC),
            _ => Err(format!("invalid 16-bits register name {}", s)),
        }
    }
}

#[given(regex = r"the bytes ((?:[A-F0-9]{2,2}(?:, )?)+) at the (\w\w) position")]
async fn setup_bytes(world: &mut CpuWorld, bytes: String, reg: Reg16) {
    let bytes = bytes.split(", ").map(|byte| u8::from_str_radix(byte, 16)).collect::<Result<Vec<u8>, _>>().expect("valid bytes in hexa format");
    let mut address = reg.read_corresponding_regs(&world.cpu.registers);
    bytes.iter().for_each(|byte| {
        drop(world.bus.write(address, *byte));
        address += 1;
    });
}

#[when(regex = r"the cpu as ticked (\d+) times?")]
async fn tick_cpu(world: &mut CpuWorld, amount: usize) {
    let mut count = 0;
    for _ in 0..amount {
        count += 1;
        world.cpu.tick(&mut world.bus)
    }
    assert_eq!(count, amount);
}

#[then(regex = r"the opcode was (\w+)")]
async fn check_parsed_opcode(world: &mut CpuWorld, opcode: String) {
    use std::string::ToString;

    assert_eq!(opcode, world.cpu.controller.opcode.as_ref().unwrap().to_string());
}

#[then(regex = r"the cpu as no action left")]
async fn check_no_action_left(world: &mut CpuWorld) {
    assert_eq!(world.cpu.controller.actions.len(), 0);
}

#[then(regex = r"the (\w\w) register is set to ([A-F0-9]{1,4})")]
async fn check_reg16_value(world: &mut CpuWorld, reg: Reg16, value: String) {
    let value = u16::from_str_radix(&value, 16).expect("valid hexa value");
    let reg_value = reg.read_corresponding_regs(&world.cpu.registers);
    assert_eq!(reg_value, value, "got {:x}, wanted {:x}", reg_value, value);
}

fn main() {
    block_on(CpuWorld::run("tests/features"));
}
