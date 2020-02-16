use std::time::Instant;

use vault_of_cardboard::scryfall;

fn main() {
    let now = Instant::now();
    let sets = scryfall::sets("data/cache");
    let elapsed = now.elapsed().as_millis();

    let mut n = 0;
    for (_, set) in &sets {
        n += set.cards.len()
    }

    println!("{}:{}", n, elapsed);
}
