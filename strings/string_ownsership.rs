fn main() {
    let s = String::from("get length of this string");

    println!("{}", calc_len(s));
}

fn calc_len(s: String) -> usize {
    s.len()
}
