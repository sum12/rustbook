fn main() {
    let mut v = vec![1, 2, 3];

    v.push(42);

    for i in &v {
        //v.push(i * i); // &v is a immutable borrow, passing refs are borrow
        println!("{}", i * i);
    }

    for i in &v {
        println!("{}", i);
    }

    println!("{}", v.len());
}
