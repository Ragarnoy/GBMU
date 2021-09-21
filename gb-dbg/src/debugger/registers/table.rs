use crate::dbg_interfaces::{RegisterDebugOperations, RegisterMap};
use std::collections::HashMap;

pub struct RegisterTable {
    pub registers: HashMap<String, u16>,
}

impl RegisterTable {
    pub fn new(table: Vec<RegisterMap>) -> Self {
        let mut register_map: HashMap<String, u16> = HashMap::with_capacity(table.len());
        for register in table {
            register_map.insert(register.0, register.1.into());
        }
        Self {
            registers: register_map,
        }
    }

    pub fn update_table(&mut self, table: Vec<RegisterMap>) {
        for ((name, value), register) in self.registers.iter_mut().zip(table) {
            if register.0 == *name {
                let new_value: u16 = register.1.into();
                if *value != new_value {
                    *value = new_value;
                }
            }
        }
    }
}
