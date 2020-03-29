fn main() {
    let mut s = String::from("hello ");

    pushstr(&mut s, "world");

    println!("{}", s);
}

fn pushstr(x: &mut String, y: &str) {
    x.push_str(y);
}
