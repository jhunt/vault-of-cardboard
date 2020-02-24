use serde::{Serialize, Serializer};
use std::collections::HashMap;
use std::io;

mod errors {
    error_chain! {}
}
pub use errors::Error;
use errors::*;

use super::{raw, Persistable};

pub type Map = HashMap<String, String>;

impl Persistable for Map {
    fn from_reader<T: io::Read>(src: &mut T) -> std::result::Result<Self, io::Error> {
        let mut s = String::new();
        src.read_to_string(&mut s)?;
        Ok(serde_json::from_str(&s)?)
    }
}

#[derive(Serialize)]
pub struct Pool {
    pub sets: HashMap<String, Set>,
    pub cards: HashMap<String, OracleCard>,

    #[serde(skip_serializing)]
    pub lookup: Map,
    #[serde(skip_serializing)]
    pub prices: HashMap<String, Option<String>>,
}

impl Pool {
    pub fn new() -> Pool {
        Pool {
            sets: HashMap::new(),
            cards: HashMap::new(),

            lookup: HashMap::new(),
            prices: HashMap::new(),
        }
    }

    pub fn read(root: &str) -> Result<Pool> {
        let mut pool = Self::new();
        for (_, set) in raw::sets(root) {
            pool.add_set(&set);
        }
        Ok(pool)
    }

    pub fn enumerate(&self) -> (usize, usize, usize) {
        let mut prints = 0;
        for (_, set) in &self.sets {
            prints += set.cards.len();
        }
        (self.cards.len(), self.sets.len(), prints)
    }

    pub fn add_set(&mut self, set: &raw::Set) {
        if !self.sets.contains_key(&set.code) {
            let code = set.code.to_uppercase();
            self.sets.insert(code.to_string(), Set::from(set));

            for card in &set.cards {
                if !self.cards.contains_key(&card.oracle_id) {
                    self.cards
                        .insert(card.oracle_id.to_string(), OracleCard::from(card));
                }

                let oracle = &self.cards[&card.oracle_id];
                self.lookup
                    .insert(format!("{} {}", &code, oracle.name), card.id.to_string());

                self.prices.insert(
                    card.id.to_string(),
                    match &card.prices {
                        None => None,
                        Some(prices) => match &prices.usd {
                            None => None,
                            Some(usd) => Some(usd.to_string()),
                        },
                    },
                );
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

impl std::convert::From<&raw::Set> for Set {
    fn from(set: &raw::Set) -> Self {
        Set {
            code: set.code.to_string().to_uppercase(),
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
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
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

impl std::convert::From<&raw::Card> for OracleCard {
    fn from(card: &raw::Card) -> Self {
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
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
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

impl std::convert::From<&raw::Card> for PrintCard {
    fn from(card: &raw::Card) -> Self {
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

#[derive(Serialize)]
pub struct Card {
    pub pid: String,
    pub var: Vec<String>,
}

pub type OwnedCard = (u32, Card);
pub type Collection = Vec<OwnedCard>;

#[cfg(test)]
mod test {
    use super::super::Persistable;
    use super::Map;

    #[test]
    fn it_should_be_able_to_read_a_lookup_json() {
        let map = Map::from_file("test/lookup.json").expect("reading lookup map");
        assert_eq!(
            map.get("MIR Barbed-Back Wurm"),
            Some(&"1b96810d-72d3-4dee-a29f-cdf85ea5ce6f".to_string())
        );
        assert_eq!(
            map.get("MIR Femeref Scouts"),
            Some(&"60192ded-689b-4cc5-9293-bff52924089b".to_string())
        );
        assert_eq!(
            map.get("VIS Tin-Wing Chimera"),
            Some(&"3375dcc6-9399-48eb-9aa4-7b40c3686cc5".to_string())
        );
    }

    #[test]
    fn a_collection_should_serialize_as_json() {
        use super::{Card, Collection};
        use serde_json::json;

        let mut c = Collection::new();
        c.push((
            22,
            Card {
                pid: "mir-plains".to_string(),
                var: vec![],
            },
        ));
        c.push((
            26,
            Card {
                pid: "mir-island".to_string(),
                var: vec![],
            },
        ));
        c.push((
            21,
            Card {
                pid: "mir-swamp".to_string(),
                var: vec![],
            },
        ));
        c.push((
            24,
            Card {
                pid: "mir-mountain".to_string(),
                var: vec![],
            },
        ));
        c.push((
            23,
            Card {
                pid: "mir-forest".to_string(),
                var: vec![],
            },
        ));

        let s = json!(c).to_string();
        assert_eq!(
            s,
            r#"[[22,{"pid":"mir-plains","var":[]}],[26,{"pid":"mir-island","var":[]}],[21,{"pid":"mir-swamp","var":[]}],[24,{"pid":"mir-mountain","var":[]}],[23,{"pid":"mir-forest","var":[]}]]"#
        );
    }
}
