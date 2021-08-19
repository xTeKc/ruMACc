extern crate mac_address;

use mac_address::{
    get_mac_address,
    mac_address_by_name,
};


struct Mac {
    mac_address: Hex,
    mac_address_name: String,
}


pub fn mac_addr() {
    match get_mac_address() {
        Ok(Some(ma)) => {
            println!("MAC Address -> {}", ma);
        }
        Ok(None) => println!("No MAC Address found."),
        Err(e) => println!("{:?}", e),
    }
}


pub fn mac_name() {
    match mac_address_by_name(name) {
        Ok(Some(ma)) => {
            println!("MAC Address of {} = {}", name, ma);
        }
        Ok(None) => println!("Interface \"{}\" not found", name),
        Err(e) => println!("{:?}", e),
    }
}