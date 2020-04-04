#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let r = Rectangle {
        width: 2,
        height: 10,
    };

    println!("The area is : {}", r.area());
    println!("The rectangle is {:?}", r);
}
