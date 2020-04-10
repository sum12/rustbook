fn main() {
    let mut v = vec![1, 2, 3];

    v.push(42);

    for i in &v {
        println!("{}", i * i);
    }

    for i in &v {
        println!("{}", i);
    }

    println!("{}", v.len());
}
