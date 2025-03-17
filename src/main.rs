use std::env;
use std::process::exit;

mod isoname;

fn main() {
    let args: Vec<String>= env::args().collect();

    if args.len() == 2 && args[1].contains("0x")
    {
        exit(parse_isoname(args[1].clone()))
    }
    else
    {
        println!("Invalid Arguments!");
        print_help();
    }

    exit(1)
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

fn print_help(){
    println!(r"Usage: isoname-parser 0x[64-bit hex number]");
}
