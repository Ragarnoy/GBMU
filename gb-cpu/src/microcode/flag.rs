use crate::interfaces::WriteFlagReg;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Flag {
    pub half_carry: Option<bool>,
    pub carry: Option<bool>,
    pub negative: Option<bool>,
    pub zero: Option<bool>,
}

impl Flag {
    pub fn from_values(previous: u8, current: u8, negative_op: bool, carry: Option<bool>) -> Self {
        Self {
            half_carry: Some((previous & 0xF) > (current & 0xF)),
            carry,
            negative: Some(negative_op),
            zero: Some(current == 0),
        }
    }

    pub fn update_reg_flag<F: WriteFlagReg>(&self, flag: &mut F) {
        if let Some(hcarry) = self.half_carry {
            flag.set_half_carry(hcarry)
        }
        if let Some(carry) = self.carry {
            flag.set_carry(carry)
        }
        if let Some(negative) = self.negative {
            flag.set_subtraction(negative)
        }
        if let Some(zero) = self.zero {
            flag.set_zero(zero)
        }
    }
}
