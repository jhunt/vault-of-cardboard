use serde_json::json;
use vault_of_cardboard::{scryfall, data};

fn main() {
    let mut pool = data::Pool::new();
    for (_, set) in scryfall::sets("data/cache") {
        pool.add_set(&set);
    }
    println!("{}", json!(pool).to_string());
}
