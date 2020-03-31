fn main() {
    let a = String::from("really input string");
    let s = getfirstwords(&a);
    println!("{}", s)
}

fn getfirstwords(a: &String) -> &str {
    for (i, &b) in a.as_bytes().iter().enumerate() {
        if b == b' ' {
            return &a[..i];
        }
    }
    &a[..]
}
