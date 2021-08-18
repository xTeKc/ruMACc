extern crate mac_address;

use mac_address::get_mac_address;

pub fn mac() {
    match get_mac_address() {
        Ok(Some(ma)) => {
            println!("MAC => {}", ma);
        }
        Ok(None) => println!("No MAC address found."),
        Err(e) => println!("{:?}", e),
    }
}