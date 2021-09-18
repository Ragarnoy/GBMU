use super::Continuum;
use crate::registers::Registers;
use gb_bus::Bus;

pub fn read<B: Bus<u8>>(regs: &mut Registers, bus: &mut B, cache: &mut Vec<u8>) -> Continuum {
    let pc = &mut regs.pc;

    let res = bus.read(*pc).map_or_else(
        |e| {
            log::error!("bus read error: {:?}", e);
            Continuum::Err
        },
        |value| {
            cache.push(value);
            Continuum::Ok
        },
    );
    *pc += 1;
    res
}
