#[macro_use]
extern crate clap;
use serde_json::json;
use std::fs::{self, File};
use std::path::Path;
use std::io::prelude::*;
use std::io;
use std::time::Instant;
use serde::{Serialize, Deserialize};
use vault_of_cardboard::data::{collection, pile, Persistable};

#[derive(Serialize, Deserialize)]
pub struct Aggregate(Vec<collection::OwnedCard>, Vec<Vec<pile::Card>>);

impl Persistable for Aggregate {
    fn from_reader<T: io::Read>(src: &mut T) -> std::result::Result<Self, io::Error> {
        let mut s = String::new();
        src.read_to_string(&mut s)?;
        Ok(serde_json::from_str(&s)?)
    }
}

impl Aggregate {
}

fn main() {
    let matches = clap_app!(reconciler =>
        (version: "1.0")
        (author: "James Hunt <bugs@vaultofcardboard.com>")
        (about: "Reconciles a collection by applying outstanding patches to it.")
        (@arg FILE: +required "Path to the collection.json file to reconcile.")).get_matches();


    let path = Path::new(matches.value_of("FILE").unwrap());
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("unable to open {} to reconcile it: {}", path.to_str().unwrap(), e),
    };

    let now = Instant::now();
    let mut raw = Aggregate::from_reader(&mut file).unwrap();
    let elapsed = now.elapsed().as_millis();
    println!("parsed aggregate collection JSON in {}ms", elapsed);

    let mut collection = collection::Collection::new();
    collection.cards = raw.0;

    let now = Instant::now();
    let mut n = 0;
    for cards in raw.1 {
        n += 1;
        collection.merge(&pile::Pile{ cards: cards });
    }
    let elapsed = now.elapsed().as_millis();
    println!("reconciled collection (applying {} patches) in {}ms", n, elapsed);

    let tmp_file = path.parent().unwrap().with_file_name(format!(".{}", path.file_name().unwrap().to_str().unwrap()));
    let mut out = File::create(&tmp_file).unwrap();

    raw.0 = collection.cards;
    let now = Instant::now();
    if let Err(e) = out.write_all(json!(Aggregate(raw.0, vec![vec![]])).to_string().as_bytes()) {
        panic!("unable to write aggregate collection JSON output to {}: {}", tmp_file.to_str().unwrap(), e);
    }
    fs::rename(tmp_file, path).unwrap();
    let elapsed = now.elapsed().as_millis();
    println!("wrote reconciled collection JSON data to {} in {}ms", path.to_str().unwrap(), elapsed);
}
