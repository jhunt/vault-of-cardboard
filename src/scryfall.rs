use std::fs;
use std::io;
use std::collections::HashMap;
use serde_json;
use serde::Deserialize;

use crate::prelude::*;

#[derive(Deserialize)]
pub struct Legality {
    pub brawl: Option<String>,
    pub commander: Option<String>,
    pub duel: Option<String>,
    pub frontier: Option<String>,
    pub future: Option<String>,
    pub historic: Option<String>,
    pub legacy: Option<String>,
    pub modern: Option<String>,
    pub oldschool: Option<String>,
    pub pauper: Option<String>,
    pub penny: Option<String>,
    pub pioneer: Option<String>,
    pub standard: Option<String>,
    pub vintage: Option<String>,
}

impl Legality {
    pub fn as_str(&self) -> String {
        fn is_legal(s: &Option<String>) -> bool {
            match s {
                Some(s) => match s.as_ref() {
                    "legal" => true,
                    _ => false,
                },
                None => false,
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
    pub eur: Option<String>,
    pub usd_foil: Option<String>,
    pub tix: Option<String>,
    pub usd: Option<String>,
}

#[derive(Deserialize)]
pub struct Card {
    pub object: String,
    pub id: String,
    pub oracle_id: String,

    pub name: String,
    pub type_line: String,
    pub oracle_text: Option<String>,
    pub flavor_text: Option<String>,

    pub artist: String,
    pub illustration_id: Option<String>,

    pub released_at: String,
    pub collector_number: String,

    pub cmc: f32,
    pub mana_cost: Option<String>,

    pub legalities: Legality,
    pub purchase_uris: PurchaseURIs,
    pub set: String,
    pub set_name: String,
    pub set_type: Option<String>,
    pub set_uri: String,

    pub frame: String,
    pub frame_effects: Option<Vec<String>>,
    pub border_color: String,
    pub layout: String,

    pub tcgplayer_id: Option<u32>,
    pub multiverse_ids: Vec<u32>,
    pub mtgo_foil_id: Option<u32>,
    pub mtgo_id: Option<u32>,

    pub scryfall_uri: String,
    pub scryfall_set_uri: String,

    pub lang: String,
    pub prints_search_uri: String,
    pub highres_image: bool,

    pub foil: Option<bool>,
    pub nonfoil: Option<bool>,
    pub full_art: Option<bool>,
    pub oversized: Option<bool>,
    pub textless: Option<bool>,
    pub reprint: Option<bool>,
    pub reserved: Option<bool>,
    pub variation: Option<bool>,

    pub booster: Option<bool>,
    pub digital: Option<bool>,

    pub story_spotlight: Option<bool>,
    pub promo: Option<bool>,

    pub prices: Option<Prices>,
    pub card_back_id: Option<String>,
    pub games: Vec<String>,

    pub uri: String,
    pub related_uris: HashMap<String, String>,
    pub rulings_uri: String,
    pub image_uris: Option<ImageURIs>,

    pub rarity: String,
    pub color_identity: Vec<String>,
    pub colors: Option<Vec<String>>,

    pub power: Option<String>,
    pub toughness: Option<String>,

    pub card_faces: Option<Vec<CardFace>>
}

#[derive(Deserialize)]
pub struct CardFace {
    pub object: String,

    pub name: String,
    pub type_line: String,
    pub oracle_text: Option<String>,
    pub flavor_text: Option<String>,

    pub artist: String,
    pub illustration_id: Option<String>,

    pub mana_cost: Option<String>,

    pub image_uris: Option<ImageURIs>,

    pub colors: Option<Vec<String>>,
    pub color_indicator: Option<Vec<String>>,

    pub power: Option<String>,
    pub toughness: Option<String>,
}

#[derive(Deserialize)]
pub struct Set {
    pub object: String,
    pub id: Option<String>,

    pub code: String,
    pub name: String,
    pub set_type: Option<String>,
    pub released_at: String,
    pub card_count: u32,

    pub block: Option<String>,
    pub block_code: Option<String>,

    pub digital: bool,
    pub foil_only: bool,

    pub mtgo_code: Option<String>,
    pub tcgplayer_id: Option<u32>,

    pub icon_svg_uri: String,
    pub uri: String,
    pub search_uri: String,

    pub cards: Vec<Card>,
}

impl Persistable for Set {
    fn from_reader<T: io::Read>(src: &mut T) -> Result<Self, io::Error> {
        let mut s = String::new();
        src.read_to_string(&mut s)?;
        Ok(serde_json::from_str(&s)?)
    }
}

pub fn sets(root: &str) -> HashMap<String, Set> {
    let mut sets: HashMap<String, Set> = HashMap::new();

    for ent in fs::read_dir(root).unwrap() {
        let ent = ent.unwrap();
        let path = ent.path();

        let metadata = fs::metadata(&path).unwrap();
        if metadata.is_file() {
            let set = Set::from_file(path.to_str().unwrap()).unwrap();
            match &set.id {
                Some(id) => sets.insert(id.to_string(), set),
                None => sets.insert("NO-ID".to_string(), set),
            };
        }
    }

    sets
}
