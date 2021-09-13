use crate::dbg_interfaces::DebugRegister;
use std::collections::HashMap;

pub struct RegisterTable<T> {
    holder: T,
    pub registers: HashMap<String, u16>,
}

impl<T: DebugRegister> RegisterTable<T> {
    pub fn new(table: T) -> Self {
        let mut register_map: HashMap<String, u16> = HashMap::with_capacity(22);
        for register in table.registers() {
            register_map.insert(register.0, register.1.into());
        }
        Self {
            holder: table,
            registers: register_map,
        }
    }

    pub fn update_table(&mut self) {
        for (name, value) in self.registers.iter_mut() {
            let new = self.holder.get(name).unwrap();
            *value = new.into();
        }
    }
}
