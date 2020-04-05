#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
fn main() {
    let localhost = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("localhost is {:?}", localhost);
    println!("loopback  is {:?}", loopback);
}
