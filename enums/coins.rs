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
        Coin::Fifty(country) => {
            println!("found a fifty from {:?}", country);
        }
        _ => println!("Keine ahnung"),
    }
}

fn main() {
    sort_fifty(Coin::Fifty(Europe::Italy));
    sort_fifty(Coin::Ten);
    sort_fifty(Coin::One);
    sort_fifty(Coin::Fifty(Europe::France));
    sort_fifty(Coin::Fifty(Europe::Germany));
}
