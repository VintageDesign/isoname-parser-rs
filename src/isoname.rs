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

    pub fn set_address_arb_capable(&mut self, value: u8)
    {
        self.raw_value |= ((value as u64) << 63) & 0xA000000000000000; 
    }

    pub fn get_industry_group(&self) -> u8
    {
        return ((self.raw_value >> 60) & 0x07) as u8;
    }

    pub fn set_industry_group(&mut self, value: u8)
    {
        self.raw_value |= ((value as u64) << 60) & 0x7000000000000000; 
    }

    pub fn get_vehicle_system_instance(&self) -> u8
    {
        return ((self.raw_value >> 56) & 0x0F) as u8;
    }

    pub fn set_vehicle_system_instance(&mut self, value: u8)
    {
        self.raw_value |= ((value as u64) << 56) & 0x0F00000000000000; 
    }

    pub fn get_vehicle_system(&self) -> u8
    {
        return ((self.raw_value >> 49) & 0x7F) as u8;
    }

    pub fn set_vehicle_system(&mut self, value: u8)
    {
        self.raw_value |= ((value as u64) << 49) & 0x00FE000000000000; 
    }
    
    pub fn get_function_code(&self) -> u8
    {
        return ((self.raw_value >> 40) & 0xFF) as u8;
    }

    pub fn set_function_code(&mut self, value: u8)
    {
        self.raw_value |= ((value as u64) << 40) & 0x0000FF0000000000; 
    }

    pub fn get_function_code_instance(&self) -> u8
    {
        return ((self.raw_value >> 35) & 0x1F) as u8;
    }

    pub fn set_function_code_instance(&mut self, value: u8)
    {
        self.raw_value |= ((value as u64) << 35) & 0x000000FA00000000; 
    }

    pub fn get_ecu_instance(&self) -> u8
    {
        return ((self.raw_value >> 32) & 0x07) as u8;
    }

    pub fn set_ecu_instance(&mut self, value: u8)
    {
        self.raw_value |= ((value as u64) << 32) & 0x0000000700000000; 
    }

    pub fn get_mc_code(&self) -> u16
    {
        return ((self.raw_value >> 21) & 0x7FF) as u16;
    }

    pub fn set_mc_code(&mut self, value: u16)
    {
        self.raw_value |= ((value as u64) << 21) & 0x00000000FFE00000; 
    }

    pub fn get_serial(&self) -> u32
    {
        return (self.raw_value & 0x1FFFFF) as u32;
    }

    pub fn set_serial(&mut self, value: u32)
    {
        self.raw_value |= (value as u64) & 0x00000000001FFFF; 
    }

    pub fn get_raw_value(&self) -> u64
    {
        return self.raw_value;
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
fn set_address_arb_works() {
    let mut name = IsoName::new(0x0);
    
    name.set_address_arb_capable(1);
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
fn set_industry_group_works() {
    let mut name = IsoName::new(0x0);
    
    name.set_industry_group(0x7);
    let result = name.get_industry_group();

    assert_eq!(result, 0x7);
}

#[test]
fn get_vehicle_system_instance_works() {
    let name = IsoName{raw_value: 0x0F00000000000000};
    let result = name.get_vehicle_system_instance();

    assert_eq!(result, 0x0F);
}

#[test]
fn set_vehicle_system_instance_works() {
    let mut name = IsoName::new(0x0);
    
    name.set_vehicle_system_instance(0xF);
    let result = name.get_vehicle_system_instance();

    assert_eq!(result, 0xF);
}

#[test]
fn get_vehicle_system_works() {
    let name = IsoName{raw_value: 0x00FE000000000000};
    let result = name.get_vehicle_system();

    assert_eq!(result, 0x7F);
}

#[test]
fn set_vehicle_system_works() {
    let mut name = IsoName::new(0x0);
    
    name.set_vehicle_system(0x7F);
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
fn set_function_code_works() {
    let mut name = IsoName::new(0x0);
    
    name.set_function_code(0xFF);
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
fn set_function_code_instance_works() {
    let mut name = IsoName::new(0x0);
    
    name.set_function_code_instance(0x1F);
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
fn set_ecu_instance_works() {
    let mut name = IsoName::new(0x0);
    
    name.set_ecu_instance(0x7);
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
fn set_mc_code_works() {
    let mut name = IsoName::new(0x0);
    
    name.set_mc_code(0x7FF);
    let result = name.get_mc_code();

    assert_eq!(result, 0x7FF);
}

#[test]
fn get_serial_works() {
    let name = IsoName{raw_value: 0x00000000001FFFFF};
    let result = name.get_serial();

    assert_eq!(result, 0x1FFFFF);
}

#[test]
fn set_serial_works() {
    let mut name = IsoName::new(0x0);
    
    name.set_serial(0x1FFFF);
    let result = name.get_serial();

    assert_eq!(result, 0x1FFFF);
}

}
