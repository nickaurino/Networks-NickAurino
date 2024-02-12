fn main() {
    use local_ip_address::local_ip;

    let my_local_ip = local_ip().unwrap();
    
    println!("Local IP: {:?}", my_local_ip);
}