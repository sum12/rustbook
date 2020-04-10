fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(42);

    for i in &v {
        println!("{}", i * i);
    }

    for i in &v {
        println!("{}", i);
    }

    println!("{}", v.len());
}
