use serde::{Serialize, Serializer};
use std::collections::HashMap;
use std::convert::From;

use super::scryfall;

#[derive(Serialize)]
pub struct Pool {
    pub sets: HashMap<String, Set>,
    pub cards: HashMap<String, OracleCard>,
}

impl Pool {
    pub fn new() -> Pool {
        Pool {
            sets: HashMap::new(),
            cards: HashMap::new(),
        }
    }

    pub fn add_set(&mut self, set: &scryfall::Set) {
        if !self.sets.contains_key(&set.code) {
            self.sets.insert(set.code.to_string(), Set::from(set));

            for card in &set.cards {
                if !self.cards.contains_key(&card.oracle_id) {
                    self.cards
                        .insert(card.oracle_id.to_string(), OracleCard::from(card));
                }
            }
        }
    }
}

#[derive(Serialize)]
pub struct Set {
    pub code: String,
    pub name: String,
    pub released_at: String,

    pub cards: Vec<PrintCard>,
}

impl From<&scryfall::Set> for Set {
    fn from(set: &scryfall::Set) -> Self {
        Set {
            code: set.code.to_string(),
            name: set.name.to_string(),
            released_at: set.released_at.to_string(),
            cards: set.cards.iter().map(|c| PrintCard::from(c)).collect(),
        }
    }
}

pub struct Legality {
    pub brawl: bool,
    pub commander: bool,
    pub duel: bool,
    pub frontier: bool,
    pub future: bool,
    pub historic: bool,
    pub legacy: bool,
    pub modern: bool,
    pub old_school: bool,
    pub pauper: bool,
    pub penny: bool,
    pub pioneer: bool,
    pub standard: bool,
    pub vintage: bool,
}

impl Legality {
    pub fn pack(&self) -> String {
        let mut s = String::new();

        macro_rules! pack {
            (encode $ok: expr => $c: expr) => {{
                if $ok {
                    s.push($c);
                }
            }};

            (encode $ok: expr => $c: expr, $(encode $ok1: expr => $c1: expr),+) => {{
                pack!(encode $ok => $c);
                pack!($(encode $ok1 => $c1),+);
            }};
        }
        pack! {
            encode self.brawl      => 'B',
            encode self.commander  => 'E',
            encode self.duel       => 'd',
            encode self.frontier   => 'j',
            encode self.future     => 'f',
            encode self.historic   => 'h',
            encode self.legacy     => 'l',
            encode self.modern     => 'm',
            encode self.old_school => 'o',
            encode self.pauper     => 'B',
            encode self.penny      => '$',
            encode self.pioneer    => 'p',
            encode self.standard   => 's',
            encode self.vintage    => 'v'
        };

        s
    }
}

impl Serialize for Legality {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.pack())
    }
}

#[derive(Serialize)]
pub struct OracleCard {
    pub id: String,
    pub name: String,
    pub type_line: String,

    pub text: String,
    pub cmc: f32,
    pub mana_cost: String,

    pub legal: Legality,

    pub color_identity: Vec<String>,
    pub colors: Vec<String>,
}

impl From<&scryfall::Card> for OracleCard {
    fn from(card: &scryfall::Card) -> Self {
        OracleCard {
            id: card.oracle_id.to_string(),
            name: card.name.to_string(),
            type_line: card.type_line.to_string(),
            text: match &card.oracle_text {
                Some(s) => s.to_string(),
                None => "".to_string(),
            },
            cmc: card.cmc,
            mana_cost: match &card.mana_cost {
                Some(s) => s.to_string(),
                None => "".to_string(),
            },
            legal: Legality {
                brawl: maybe_legal(&card.legalities.brawl),
                commander: maybe_legal(&card.legalities.commander),
                duel: maybe_legal(&card.legalities.duel),
                frontier: maybe_legal(&card.legalities.frontier),
                future: maybe_legal(&card.legalities.future),
                historic: maybe_legal(&card.legalities.historic),
                legacy: maybe_legal(&card.legalities.legacy),
                modern: maybe_legal(&card.legalities.modern),
                old_school: maybe_legal(&card.legalities.oldschool),
                pauper: maybe_legal(&card.legalities.pauper),
                penny: maybe_legal(&card.legalities.penny),
                pioneer: maybe_legal(&card.legalities.pioneer),
                standard: maybe_legal(&card.legalities.standard),
                vintage: maybe_legal(&card.legalities.vintage),
            },
            color_identity: card.color_identity.clone(),
            colors: match &card.colors {
                Some(l) => l.clone(),
                None => vec![],
            },
        }
    }
}

pub struct Flags {
    pub full_art: bool,
    pub oversized: bool,
    pub reprint: bool,
    pub reserved: bool,
    pub variation: bool,
    pub story_spotlight: bool,

    pub rarity: String,
}

impl Flags {
    pub fn pack(&self) -> String {
        let mut s = String::new();

        macro_rules! pack {
            (encode $ok: expr => $c: expr) => {{
                if $ok {
                    s.push($c);
                }
            }};

            (encode $ok: expr => $c: expr, $(encode $ok1: expr => $c1: expr),+) => {{
                pack!(encode $ok => $c);
                pack!($(encode $ok1 => $c1),+);
            }};
        }
        pack! {
            encode self.full_art        => '^',
            encode self.oversized       => 'O',
            encode self.reprint         => '+',
            encode self.reserved        => '!',
            encode self.variation       => '~',
            encode self.story_spotlight => '@'
        };

        match self.rarity.as_str() {
            "common" => s.push('1'),
            "uncommon" => s.push('2'),
            "rare" => s.push('3'),
            "mythic" => s.push('4'),
            _ => (),
        };

        s
    }
}

impl Serialize for Flags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.pack())
    }
}

#[derive(Serialize)]
pub struct PrintCard {
    pub id: String,
    pub oid: String,

    pub artist: String,
    pub number: String,

    pub frame: String,
    pub border: String,
    pub layout: String,

    pub flags: Flags,
}

fn maybe_true(t: &Option<bool>) -> bool {
    match t {
        Some(true) => true,
        _ => false,
    }
}

fn maybe_legal(s: &Option<String>) -> bool {
    match s {
        Some(s) => s == "legal",
        _ => false,
    }
}

impl From<&scryfall::Card> for PrintCard {
    fn from(card: &scryfall::Card) -> Self {
        PrintCard {
            id: card.id.to_string(),
            oid: card.oracle_id.to_string(),
            artist: card.artist.to_string(),
            number: card.collector_number.to_string(),
            frame: card.frame.to_string(),
            border: card.border_color.to_string(),
            layout: card.layout.to_string(),

            flags: Flags {
                full_art: maybe_true(&card.full_art),
                oversized: maybe_true(&card.oversized),
                reprint: maybe_true(&card.reprint),
                reserved: maybe_true(&card.reserved),
                variation: maybe_true(&card.variation),
                story_spotlight: maybe_true(&card.story_spotlight),
                rarity: card.rarity.to_string(),
            },
        }
    }
}
