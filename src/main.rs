use std::env;
use std::process::exit;


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
        println!("Address Arbitration Capable: \t{}",  isoname_integer >> 63);
        println!("Industry Group: \t\t{}",  (isoname_integer >> 60) & 0x007);
        println!("Vehicle System Instance: \t{}",  (isoname_integer >> 56) & 0x00F);
        println!("Vehicle System (Device Class): \t{}",  (isoname_integer >> 49) & 0x007F);
        println!("Function Code: \t\t\t{}",  (isoname_integer >> 40) & 0x00FF);
        println!("Function Instance: \t\t{}",  (isoname_integer >> 35) & 0x001F);
        println!("ECU Instance: \t\t\t{}",  (isoname_integer >> 32) & 0x007);
        println!("MC Code: \t\t\t{}",  (isoname_integer >> 21) & 0x7FF);
        println!("Product Family: \t\t{}",  (isoname_integer >> 16) & 0x1F);
        println!("ID Number: \t\t\t{}",  (isoname_integer) & 0xFFFF);
        
        0
    }
    else{
        println!("Error parsing {full_isoname} as an isoname!");
        1
    }
}

fn print_help(){
    println!(r"Usage: isoname-parser 0x[64-bit hex number]");
}
