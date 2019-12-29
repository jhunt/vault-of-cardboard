use std::fs::File;
use std::io;
use std::io::prelude::*;

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Legality {
    pub legacy: String,
    pub oldschool: String,
    pub vintage: String,
    pub pauper: String,
    pub duel: String,
    pub standard: String,
    pub future: String,
    pub frontier: String,
    pub modern: String,
    pub commander: String,
    pub penny: String,
}

impl Legality {
    pub fn as_str(&self) -> String {
        fn is_legal(s: &String) -> bool {
            match s.as_ref() {
                "legal" => true,
                _ => false,
            }
        }

        let mut split = "";
        let mut s = String::new();

        if is_legal(&self.legacy) {
            s.push_str(split);
            s.push_str("legacy");
            split = ", ";
        }

        if is_legal(&self.oldschool) {
            s.push_str(split);
            s.push_str("oldschool");
            split = ", ";
        }

        if is_legal(&self.vintage) {
            s.push_str(split);
            s.push_str("vintage");
            split = ", ";
        }

        if is_legal(&self.pauper) {
            s.push_str(split);
            s.push_str("pauper");
            split = ", ";
        }

        if is_legal(&self.duel) {
            s.push_str(split);
            s.push_str("duel");
            split = ", ";
        }

        if is_legal(&self.standard) {
            s.push_str(split);
            s.push_str("standard");
            split = ", ";
        }

        if is_legal(&self.future) {
            s.push_str(split);
            s.push_str("future");
            split = ", ";
        }

        if is_legal(&self.frontier) {
            s.push_str(split);
            s.push_str("frontier");
            split = ", ";
        }

        if is_legal(&self.modern) {
            s.push_str(split);
            s.push_str("modern");
            split = ", ";
        }

        if is_legal(&self.commander) {
            s.push_str(split);
            s.push_str("commander");
            split = ", ";
        }

        if is_legal(&self.penny) {
            s.push_str(split);
            s.push_str("penny");
            split = ", ";
        }

        let _ = split;

        s
    }
}

#[derive(Deserialize)]
pub struct PurchaseURIs {
    pub tcgplayer: String,
    pub cardhoarder: String,
    pub cardmarket: String,
}

#[derive(Deserialize)]
pub struct ImageURIs {
    pub png: String,
    pub small: String,
    pub art_crop: String,
    pub normal: String,
    pub large: String,
    pub border_crop: String,
}

#[derive(Deserialize)]
pub struct Prices {
    pub eur: String,
    pub usd_foil: Option<String>,
    pub tix: String,
    pub usd: String,
}

#[derive(Deserialize)]
pub struct Card {
    pub object: String,
    pub id: String,
    pub oracle_id: String,

    pub name: String,
    pub type_line: String,
    pub oracle_text: String,
    pub flavor_text: Option<String>,

    pub artist: String,
    pub illustration_id: String,

    pub released_at: String,
    pub collector_number: String,

    pub cmc: f32,
    pub mana_cost: String,

    pub legalities: Legality,
    pub purchase_uris: PurchaseURIs,
    pub set: String,
    pub set_name: String,
    pub set_type: String,
    pub set_uri: String,

    pub frame: String,
    pub border_color: String,
    pub layout: String,

    pub tcgplayer_id: u32,
    pub multiverse_ids: Vec<u32>,
    pub mtgo_foil_id: u32,
    pub mtgo_id: u32,

    pub scryfall_uri: String,
    pub scryfall_set_uri: String,

    pub lang: String,
    pub prints_search_uri: String,
    pub highres_image: bool,

    pub foil: bool,
    pub nonfoil: bool,
    pub full_art: bool,
    pub oversized: bool,
    pub textless: bool,
    pub reprint: bool,
    pub reserved: bool,
    pub variation: bool,

    pub booster: bool,
    pub digital: bool,

    pub story_spotlight: bool,
    pub promo: bool,

    pub prices: Prices,
    pub card_back_id: String,
    pub games: Vec<String>,

    pub uri: String,
    pub related_uris: HashMap<String, String>,
    pub rulings_uri: String,
    pub image_uris: ImageURIs,

    pub rarity: String,
    pub color_identity: Vec<String>,
    pub colors: Vec<String>,
}

#[derive(Deserialize)]
pub struct Set {
    pub object: String,
    pub id: String,

    pub code: String,
    pub name: String,
    pub set_type: String,
    pub released_at: String,
    pub card_count: u32,

    pub block: String,
    pub block_code: String,

    pub digital: bool,
    pub foil_only: bool,

    pub mtgo_code: String,
    pub tcgplayer_id: u32,

    pub icon_svg_uri: String,
    pub uri: String,
    pub search_uri: String,

    pub cards: Vec<Card>,
}

impl Set {
    pub fn from_file(file: &str) -> Result<Set, io::Error> {
        let mut s = String::new();
        File::open(file)?.read_to_string(&mut s)?;
        Ok(serde_json::from_str(&s)?)

        /*
        let mut file = match File::open("data/cache/mir.set") {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open mir: {}", why.description()),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        let s = match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read mir: {}", why.description()),
            Ok(_) => s,
        };

        let set: scryfall::data::Set = match serde_json::from_str(&s) {
            Err(why) => {
                if why.is_syntax() {
                    panic!(
                        "couldn't parse mir: (syntax error on line {}, column {}): {}",
                        why.line(),
                        why.column(),
                        why.description()
                    );
                } else if why.is_data() {
                    panic!(
                        "couldn't parse mir: (semantic error on line {}, column {}): {}",
                        why.line(),
                        why.column(),
                        why.description()
                    );
                } else {
                    panic!("couldn't parse mir: {}", why.description());
                }
            }
            Ok(set) => set,
        };
        */
    }
}
