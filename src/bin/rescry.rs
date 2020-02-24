#[macro_use]
extern crate clap;
use serde_json::json;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use vault_of_cardboard::data::pool::Pool;

fn main() {
    let matches = clap_app!(rescry =>
        (version: "1.0")
        (author: "James Hunt <bugs@vaultofcardboard.com>")
        (about: "Transforms ingested Scryfall set/card data into usable formats.")
        (@arg raw:    -r --raw    +takes_value +required "Where to find the raw Scryfall set files.")
        (@arg cards:  -c --cards  +takes_value "Where to put the output cards.json file.")
        (@arg prices: -p --prices +takes_value "Where to put the output prices.json file.")
        (@arg lookup: -l --lookup +takes_value "Where to put the output lookup.json file.")).get_matches();

    let cache = match matches.value_of("raw") {
        Some(cache) => cache,
        None => "data/cache",
    };

    let cards_json = match matches.value_of("cards") {
        Some(v) => v,
        None => "cards.json",
    };
    let mut cards = match File::create(cards_json) {
        Ok(f) => f,
        Err(e) => panic!(
            "unable to open {} for writing cards JSON output: {}",
            cards_json, e
        ),
    };

    let prices_json = match matches.value_of("prices") {
        Some(v) => v,
        None => "prices.json",
    };
    let mut prices = match File::create(prices_json) {
        Ok(f) => f,
        Err(e) => panic!(
            "unable to open {} for writing prices JSON output: {}",
            prices_json, e
        ),
    };

    let lookup_json = match matches.value_of("lookup") {
        Some(v) => v,
        None => "lookup.json",
    };
    let mut lookup = match File::create(lookup_json) {
        Ok(f) => f,
        Err(e) => panic!(
            "unable to open {} for writing lookup JSON output: {}",
            lookup_json, e
        ),
    };

    let now = Instant::now();
    let pool = Pool::read(cache).unwrap();
    let elapsed = now.elapsed().as_millis();
    let (no, ns, nc) = pool.enumerate();
    println!("parsed raw scryfall data ({} oracle cards / {} sets / {} print cards) in {}ms", no, ns, nc, elapsed);

    let now = Instant::now();
    if let Err(e) = cards.write_all(json!(pool).to_string().as_bytes()) {
        panic!("unable to write cards JSON output to {}: {}", cards_json, e);
    }
    let elapsed = now.elapsed().as_millis();
    println!("write cards JSON data to {} in {}ms", cards_json, elapsed);

    let now = Instant::now();
    if let Err(e) = prices.write_all(json!(pool.prices).to_string().as_bytes()) {
        panic!("unable to write prices JSON output to {}: {}", prices_json, e);
    }
    let elapsed = now.elapsed().as_millis();
    println!("write prices JSON data to {} in {}ms", prices_json, elapsed);

    let now = Instant::now();
    if let Err(e) = lookup.write_all(json!(pool.lookup).to_string().as_bytes()) {
        panic!("unable to write lookup JSON output to {}: {}", lookup_json, e);
    }
    let elapsed = now.elapsed().as_millis();
    println!("write lookup JSON data to {} in {}ms", lookup_json, elapsed);
}
