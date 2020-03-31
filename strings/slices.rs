fn main() {
    let a = String::from("input string");
    let s = getfirstwords(&a);
    println!("{}", s)
}

fn getfirstwords(a: &String) -> &str {
    &a[..2]
}
