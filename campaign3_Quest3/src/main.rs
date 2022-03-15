enum Coin {
    Heads,
    Tails,
}
fn coinflip(coin: Coin) -> u8 {
    match coin {
        Coin::Heads => {
            println!("Coin Flipped! Heads!");
            0
        },
        Coin::Tails => {
            println!("Coin Flipped! Tails!");
            1
        }
    }
}
fn main() {
    coinflip(Coin::Heads);
    coinflip(Coin::Tails);
}
//

