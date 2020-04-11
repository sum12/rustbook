fn main() {
    let mut a: String = String::from("abc");
    let b: &str = "def";

    a.push_str("xyz"); // String is mutable

    let mut c = a + b; // b is already ref, but still need to pass ref to ref
    c.push_str(b); // is already a ref, but still need to pass ref to ref

    //println!("{}", a); //a is moved to c, and extended to include &b
    println!("{}", b);
    println!("{}", c);
}
