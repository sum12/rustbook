#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(r: &Rectangle) -> u32 {
    r.width * r.height
}
fn main() {
    let r = Rectangle {
        width: 2,
        height: 10,
    };

    println!("The area is : {}", area(&r));
    println!("The rectangle is {:?}", r);
}
