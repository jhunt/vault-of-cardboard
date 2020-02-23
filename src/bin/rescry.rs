use serde_json::json;
use vault_of_cardboard::data::{self, raw};

fn main() {
    let mut pool = data::Pool::new();
    for (_, set) in raw::sets("data/cache") {
        pool.add_set(&set);
    }
    println!("{}", json!(pool).to_string());
}
