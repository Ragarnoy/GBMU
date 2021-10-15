use async_trait::async_trait;
use cucumber::{given, then, when, World, WorldInit};
use futures::executor::block_on;
use gb_bus::Bus;
use gb_clock::Ticker;
use gb_cpu::cpu::Cpu;
use gb_test::{MockBus, Reg16};
use std::{
    convert::Infallible,
    fmt::{self, Debug},
};

#[derive(Clone, WorldInit, Default)]
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

#[given(regex = r"the bytes ((?:[A-F0-9]{2,2}(?:, )?)+) at the position (\w\w)")]
async fn setup_bytes(world: &mut CpuWorld, bytes: String, reg: Reg16) {
    let bytes = bytes
        .split(", ")
        .map(|byte| u8::from_str_radix(byte, 16))
        .collect::<Result<Vec<u8>, _>>()
        .expect("valid bytes in hexa format");
    let mut address = reg.read_corresponding_regs(&world.cpu.registers);
    bytes.iter().for_each(|byte| {
        drop(world.bus.write(address, *byte));
        address += 1;
    });
}

#[when(regex = r"the cpu has ticked (\d+) times?")]
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

    assert_eq!(
        opcode,
        world.cpu.controller.opcode.as_ref().unwrap().to_string()
    );
}

#[then(regex = r"the cpu has no action left")]
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
