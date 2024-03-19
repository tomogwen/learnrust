// enums rust book

enum IpAddress {
    V4(String),
    V6(String)
}

fn main() {
    let home = IpAddress::V4(String::from("127.0.0.1"));
    let loopback = IpAddress::V6(String::from("::1"));
    

        
}
