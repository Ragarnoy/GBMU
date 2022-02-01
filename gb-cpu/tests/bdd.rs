use async_trait::async_trait;
use cucumber::{gherkin::Step, given, then, when, World, WorldInit};
use futures::executor::block_on;
use gb_bus::Bus;
use gb_clock::Ticker;
use gb_cpu::cpu::Cpu;
use gb_test::{MockBus, Reg16, Reg8};
use std::{
    convert::Infallible,
    fmt::{self, Debug},
};

#[derive(WorldInit, Default)]
struct CpuWorld {
    cpu: Cpu,
    bus: MockBus,
}

impl Debug for CpuWorld {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CpuWorld {{ cpu: {:x?}, bus: {:x?} }}",
            self.cpu, self.bus
        )
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
        drop(world.bus.write(address, *byte, None));
        address += 1;
    });
}

#[given(regex = r"the register (\w\w) set to the value ([A-F0-9]{1,4})")]
async fn setup_register(world: &mut CpuWorld, reg: Reg16, value: String) {
    let value = u16::from_str_radix(&value, 16).expect("valid hexa value");
    reg.write_corresponding_regs(&mut world.cpu.registers, value);
}

#[given(regex = r"the u8 register (\w) set to the value ([A-F0-9]{1,2})")]
async fn setup_u8_register(world: &mut CpuWorld, reg: Reg8, value: String) {
    let value = u8::from_str_radix(&value, 16).expect("valid hexa value");
    reg.write_corresponding_regs(&mut world.cpu.registers, value);
}

#[given(regex = r"the flag ([\w ]+) is (re)?set")]
async fn set_flag(world: &mut CpuWorld, flag: String, toggle: String) {
    use gb_cpu::interfaces::WriteFlagReg;

    let toggle = toggle.is_empty();
    match flag.as_str() {
        "zero" => world.cpu.registers.set_zero(toggle),
        "half carry" => world.cpu.registers.set_half_carry(toggle),
        "carry" => world.cpu.registers.set_carry(toggle),
        "subtraction" => world.cpu.registers.set_subtraction(toggle),
        _ => panic!("invalid flag name {}", flag),
    }
}

#[given(regex = r"the flag ([\w ]+) is toggle")]
async fn toggle_flag(world: &mut CpuWorld, flag: String) {
    use gb_cpu::interfaces::{ReadFlagReg, WriteFlagReg};

    match flag.as_str() {
        "zero" => world.cpu.registers.set_zero(!world.cpu.registers.zero()),
        "half carry" => world
            .cpu
            .registers
            .set_half_carry(!world.cpu.registers.half_carry()),
        "carry" => world.cpu.registers.set_carry(!world.cpu.registers.carry()),
        "subtraction" => world
            .cpu
            .registers
            .set_subtraction(!world.cpu.registers.subtraction()),
        _ => panic!("invalid flag name {}", flag),
    }
}

#[given("the cpu is reset")]
async fn reset_cpu(world: &mut CpuWorld) {
    world.cpu = Cpu::default();
}

#[given("the following bytes")]
async fn write_bytes(world: &mut CpuWorld, step: &Step) {
    let table = step.table.as_ref().expect("missing data table");
    let mut rows = table.rows.iter();
    rows.next();
    for row in rows {
        let address = u16::from_str_radix(&row[0], 16).expect("valid hexa value");
        let value = u8::from_str_radix(&row[1], 16).expect("valid hexa value");
        world
            .bus
            .write(address, value, None)
            .expect("could not write value at address in bus")
    }
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
    assert_eq!(world.cpu.controller.current_cycle.len(), 0);
}

#[then(regex = r"the register (\w\w) is set to ([A-F0-9]{1,4})")]
async fn check_reg16_value(world: &mut CpuWorld, reg: Reg16, value: String) {
    let value = u16::from_str_radix(&value, 16).expect("valid hexa value");
    let reg_value = reg.read_corresponding_regs(&world.cpu.registers);
    assert_eq!(reg_value, value, "got {:x}, wanted {:x}", reg_value, value);
}

#[then(
    regex = r"the composite register ((?:\w) (?:\w)) set to the value ((?:[A-F0-9]{1,2}) (?:[A-F0-9]{1,2}))"
)]
async fn check_composite_reg_value(world: &mut CpuWorld, reg: String, value: String) {
    use std::str::FromStr;

    let reg = reg.split(' ').collect::<Vec<&str>>().join("");
    let reg = Reg16::from_str(&reg).expect("cannot decode composite register");

    let value = value.split(' ').collect::<Vec<&str>>().join("");
    let value = u16::from_str_radix(&value, 16).expect("valid hexa value");

    let reg_value = reg.read_corresponding_regs(&world.cpu.registers);
    assert_eq!(reg_value, value, "got {:x}, wanted {:x}", reg_value, value);
}

#[then(regex = r"the u8 register (\w) is set to ([A-F0-9]{1,2})")]
async fn check_reg8_value(world: &mut CpuWorld, reg: Reg8, value: String) {
    let value = u8::from_str_radix(&value, 16).expect("valid hexa value");
    let reg_value = reg.read_corresponding_regs(&world.cpu.registers);
    assert_eq!(reg_value, value, "got {:x}, wanted {:x}", reg_value, value);
}

#[then(regex = r"the values written at ([A-F0-9]{1,4}) are ((?:[A-F0-9]{2,2}(:?, )?)+)")]
async fn check_u16_in_bus(world: &mut CpuWorld, address: String, values: String) {
    let address = u16::from_str_radix(&address, 16).expect("valid hexa value");
    let values = values
        .split(", ")
        .map(|value| u8::from_str_radix(value, 16))
        .collect::<Result<Vec<u8>, _>>()
        .expect("valid hexa values");

    for (index, value) in values.iter().enumerate() {
        let addr = address + index as u16;
        let res = world.bus.read(addr, None);
        assert_eq!(
            Ok(*value),
            res,
            "invalid value for index {} (address: {:x})",
            index,
            addr
        );
    }
}

#[then(regex = r"the flag ([\w ]+) is (not )?set")]
async fn check_flag(world: &mut CpuWorld, flag: String, toggle: String) {
    use gb_cpu::interfaces::ReadFlagReg;

    let toggle = toggle.is_empty();
    let flag = match flag.as_str() {
        "zero" => world.cpu.registers.zero(),
        "half carry" => world.cpu.registers.half_carry(),
        "carry" => world.cpu.registers.carry(),
        "subtraction" => world.cpu.registers.subtraction(),
        _ => panic!("invalid flag name {}", flag),
    };
    assert_eq!(toggle, flag);
}

#[then(regex = r"the cpu has ticked (\d+) times for the current (prefixed )?opcode (\w+)")]
async fn check_opcode_duration(
    world: &mut CpuWorld,
    mut count: usize,
    prefixed: String,
    opcode: String,
) {
    if !prefixed.is_empty() {
        world.cpu.tick(&mut world.bus);
        let current_opcode = world.cpu.controller.opcode.as_ref().unwrap().to_string();
        assert_eq!("PrefixCb", current_opcode,);
        count -= 1;
    }
    for i in 0..count {
        world.cpu.tick(&mut world.bus);
        let current_opcode = world.cpu.controller.opcode.as_ref().unwrap().to_string();
        assert_eq!(
            opcode, current_opcode,
            "at tick {}, current opcode = {}, expected = {}",
            i, current_opcode, opcode
        );
        if i == count - 1 {
            assert!(
                world.cpu.controller.is_instruction_finished,
                "opcode should be finished, {} cycles remaining",
                world.cpu.controller.cycles.len()
            );
        } else {
            assert!(
                !world.cpu.controller.is_instruction_finished,
                "at tick {} opcode finished",
                i
            );
        }
    }
}

fn main() {
    block_on(CpuWorld::run("tests/features"));
}
