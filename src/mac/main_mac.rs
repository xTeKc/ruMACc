use mac_address::get_mac_address;

pub fn mac_addr() {
    match get_mac_address() {
        Ok(Some(ma)) => {
            println!("MAC Address -> {}", ma);
        }
        Ok(None) => println!("No MAC Address found."),
        Err(e) => println!("{:?}", e),
    }
}

