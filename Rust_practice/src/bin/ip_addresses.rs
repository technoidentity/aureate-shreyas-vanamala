#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

fn route(address: IpAddr) {
    match address {
        IpAddr::V4(value) => println!("IPv4: {value}"),
        IpAddr::V6(value) => println!("IPv6: {value}"),
    }
}

fn main() {
    println!("Address kinds: {:?}, {:?}", IpAddrKind::V4, IpAddrKind::V6);

    route(IpAddr::V4(String::from("127.0.0.1")));
    route(IpAddr::V6(String::from("::1")));
}
