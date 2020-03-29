fn main() {
    let s = String::from("get length of this string");

    println!("{}", calc_len(&s));

    let (x, s) = ret_tuple(s);

    println!("length from tuple {}", x);

    println!("{}", s);
}

fn calc_len(s: &String) -> usize {
    s.len()
}

fn ret_tuple(x: String) -> (usize, String) {
    (x.len(), x)
}
