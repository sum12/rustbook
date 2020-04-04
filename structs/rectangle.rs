#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let r = Rectangle {
        width: 2,
        height: 10,
    };

    println!("The rectangle is {:?}", r);
}
