use std::env;
use std::process::exit;
use text_io::read;

mod isoname;

fn main() {
    let args: Vec<String>= env::args().collect();

    let mut valid_args = false; 
    let mut return_code = 1;
    if args.len() == 2
    {
        if args[1].contains("0x")
        {
            valid_args = true;
            return_code = parse_isoname(args[1].clone());
        }
        else if args[1] == "--pack-name" || args[1] == "-p"
        {
            valid_args = true;
            pack_isoname();
        }
        else if args[1] == "--help" || args[1] == "-h"
        {
            valid_args = true;
            print_help();
        }
    }

    if !valid_args
    {
        println!("Invalid Arguments!");
        print_help();
    }

    exit(return_code)
}


fn parse_isoname(full_isoname: String) -> i32
{
    if let Ok(isoname_integer) = u64::from_str_radix(full_isoname.trim_start_matches("0x"), 16)
    {
        let name = isoname::IsoName::new(isoname_integer);
        println!("Address Arbitration Capable: \t{}",  name.get_address_arb_capable());
        println!("Industry Group: \t\t{}",  name.get_industry_group());
        println!("Vehicle System Instance: \t{}",  name.get_vehicle_system_instance());
        println!("Vehicle System (Device Class): \t{}",  name.get_vehicle_system());
        println!("Function Code: \t\t\t{}",  name.get_function_code());
        println!("Function Instance: \t\t{}",  name.get_function_code_instance());
        println!("ECU Instance: \t\t\t{}",  name.get_ecu_instance());
        println!("MC Code: \t\t\t{}",  name.get_mc_code());
        println!("Serial: \t\t\t{}", name.get_serial());
        
        return 0;
    }
    else{
        println!("Error parsing {full_isoname} as an isoname!");
        return 1;
    }
}

fn pack_isoname()
{
        let mut name = isoname::IsoName::new(0);

        print!("Address Arbitration Capable: ");
        let address_arb_capable: u8 = read!();
        name.set_address_arb_capable(address_arb_capable);

        print!("Industry Group: ");
        let industry_group: u8 = read!();
        name.set_industry_group(industry_group);

        print!("Vehicle System (Device Class) Instance: ");
        let vehicle_system_instance: u8 = read!();
        name.set_vehicle_system_instance(vehicle_system_instance);

        print!("Vehicle System (Device Class): ");
        let vehicle_system: u8 = read!();
        name.set_vehicle_system(vehicle_system);

        print!("Function Code: ");
        let function_code: u8 = read!();
        name.set_function_code(function_code);

        print!("Function Code Instance: ");
        let function_code_instance: u8 = read!();
        name.set_function_code_instance(function_code_instance);

        print!("ECU Instance: ");
        let ecu_instance: u8 = read!();
        name.set_ecu_instance(ecu_instance);

        print!("Manufacturer Code: ");
        let mc_code: u16 = read!();
        name.set_mc_code(mc_code);
        
        print!("Serial Number: ");
        let serial: u32 = read!();
        name.set_serial(serial);

        println!("Isoname: 0x{:X}", name.get_raw_value());
}

fn print_help(){
    println!(r"Usage: isoname-parser [0x<64-bit hex number>] [options]
Possible Options:
    --pack-name|-p  Packs the ISOName based on the individual fields");
}
