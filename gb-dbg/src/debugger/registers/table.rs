use crate::dbg_interfaces::{RegisterDebugOperations, RegisterMap};
use std::collections::HashMap;

pub struct RegisterTable<T> {
    pub registers: HashMap<String, u16>,
}

impl<T: RegisterDebugOperations> RegisterTable<T> {
    pub fn new(table: Vec<RegisterMap>) -> Self {
        let mut register_map: HashMap<String, u16> = HashMap::with_capacity(table.len());
        for register in table {
            register_map.insert(register.0, register.1.into());
        }
        Self {
            registers: register_map,
        }
    }

    pub fn update_table(&mut self, holder: T) {
        for (name, value) in self.registers.iter_mut() {
            let new = holder.cpu_get(name).unwrap();
            *value = new.into();
        }
    }
}
