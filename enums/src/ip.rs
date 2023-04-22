#[derive(Debug)]
enum IpAddressKind {
    V4,
    V6,
}

#[derive(Debug)]
#[allow(dead_code)]
struct IpAddress {
    kind: IpAddressKind,
    address: String,
}

pub fn ip() {
    let four = IpAddressKind::V4;
    let size = IpAddressKind::V6;

    let home = IpAddress {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddress {
        kind: IpAddressKind::V6,
        address: String::from("::1"),
    };

    println!("Four: {:?}", four);
    println!("Size: {:?}", size);
    println!("-------------------");
    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);
}
