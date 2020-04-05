#[derive(Debug)]
enum Europe {
    Germany,
    France,
    Italy,
}

enum Coin {
    One,
    Ten,
    Fifty(Europe),
}

fn sort_fifty(coin: Coin) {
    match coin {
        Coin::One => (),
        Coin::Ten => (),
        Coin::Fifty(country) => {
            println!("found a fifty from {:?}", country);
        }
    }
}

fn main() {
    sort_fifty(Coin::Fifty(Europe::Italy));
    sort_fifty(Coin::Ten);
}
