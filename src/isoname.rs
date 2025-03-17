#[repr(transparent)]

pub struct IsoName{
    raw_value: u64,
}

impl IsoName{
    pub fn new(value: u64) -> Self {
        return Self{raw_value: value};
    }
    pub fn get_address_arb_capable(&self) -> u8
    {
        return (self.raw_value >> 63) as u8;
    }

    pub fn get_industry_group(&self) -> u8
    {
        return ((self.raw_value >> 60) & 0x07) as u8;
    }

    pub fn get_vehicle_system_instance(&self) -> u8
    {
        return ((self.raw_value >> 56) & 0x0F) as u8;
    }

    pub fn get_vehicle_system(&self) -> u8
    {
        return ((self.raw_value >> 49) & 0x7F) as u8;
    }
    
    pub fn get_function_code(&self) -> u8
    {
        return ((self.raw_value >> 40) & 0xFF) as u8;
    }

    pub fn get_function_code_instance(&self) -> u8
    {
        return ((self.raw_value >> 35) & 0x1F) as u8;
    }

    pub fn get_ecu_instance(&self) -> u8
    {
        return ((self.raw_value >> 32) & 0x07) as u8;
    }

    pub fn get_mc_code(&self) -> u16
    {
        return ((self.raw_value >> 21) & 0x7FF) as u16;
    }

    pub fn get_serial(&self) -> u32
    {
        return (self.raw_value & 0x1FFFFF) as u32;
    }
}

#[cfg(test)]

mod tests {
use super::*;

#[test]
fn get_address_arb_works() {
    let name = IsoName{raw_value: 0xa000000000000000};
    let result = name.get_address_arb_capable();

    assert_eq!(result, 1);
}

#[test]
fn get_industry_group_works() {
    let name = IsoName{raw_value: 0x7000000000000000};
    let result = name.get_industry_group();

    assert_eq!(result, 0x07);
}

#[test]
fn get_vehicle_system_instance_works() {
    let name = IsoName{raw_value: 0x0F00000000000000};
    let result = name.get_vehicle_system_instance();

    assert_eq!(result, 0x0F);
}

#[test]
fn get_vehicle_system_works() {
    let name = IsoName{raw_value: 0x00FE000000000000};
    let result = name.get_vehicle_system();

    assert_eq!(result, 0x7F);
}

#[test]
fn get_function_code_works() {
    let name = IsoName{raw_value: 0x0000FF0000000000};
    let result = name.get_function_code();

    assert_eq!(result, 0xFF);
}

#[test]
fn get_function_code_instance_works() {
    let name = IsoName{raw_value: 0x000000FA00000000};
    let result = name.get_function_code_instance();

    assert_eq!(result, 0x1F);
}

#[test]
fn get_ecu_instance_works() {
    let name = IsoName{raw_value: 0x0000000700000000};
    let result = name.get_ecu_instance();

    assert_eq!(result, 0x7);
}

#[test]
fn get_mc_code_works() {
    let name = IsoName{raw_value: 0x00000000FFE00000};
    let result = name.get_mc_code();

    assert_eq!(result, 0x7FF);
}

#[test]
fn get_serial_works() {
    let name = IsoName{raw_value: 0x00000000001FFFFF};
    let result = name.get_serial();

    assert_eq!(result, 0x1FFFFF);
}

}
