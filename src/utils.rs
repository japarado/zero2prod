pub fn print_app_address(address: Option<&str>, port: u16) {
    let address = address.unwrap_or("127.0.0.1");
    println!("Application running at {}:{}", address, port);
}
