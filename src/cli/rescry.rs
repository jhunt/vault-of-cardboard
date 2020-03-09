use serde_json::json;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

use crate::card;

pub fn run(raw: Option<&str>, cards_json: Option<&str>, prices_json: Option<&str>, lookup_json: Option<&str>) {
    let raw = match raw {
        Some(v) => v,
        None => "data/cache",
    };

    let cards_json = match cards_json {
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

    let prices_json = match prices_json {
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

    let lookup_json = match lookup_json {
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
    let pool = card::Pool::read(raw).unwrap();
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
