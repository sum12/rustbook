#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let r = Rectangle {
        width: 2,
        height: 10,
    };
    let o = Rectangle {
        width: 1,
        height: 5,
    };

    println!("rectangle r: {:?}", r);
    println!("area is : {}", r.area());
    println!("rectangle o: {:?}", o);
    println!("area is : {}", o.area());

    println!("Can r hold o : {}", r.can_hold(&o));
    println!("Can o hold r : {}", o.can_hold(&r));
}
