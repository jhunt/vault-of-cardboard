use serde::{Serialize, Deserialize, Serializer};
use std::collections::HashMap;
use std::io;

use crate::prelude::*;
use crate::{cdif, scryfall};

mod errors {
    error_chain! {}
}
pub use errors::Error;
use errors::*;

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub quantity: i32,
    pub id: String,
    pub gvars: Vec<String>,
    pub lvars: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize)]
pub struct Pile {
    pub cards: Vec<Card>,
}

impl Pile {
    pub fn resolve(file: cdif::File, lookup: Map) -> Self {
        let mut cards = vec![];

        for (_, line) in file.lines {
            match lookup.get(&format!("{} {}", &line.set, &line.oracle)) {
                Some(id) => cards.push(Card {
                    id: id.to_string(),
                    quantity: line.quantity,
                    lvars: line.lvars,
                    gvars: line.gvars,
                }),
                None => panic!(format!("unable to find {} {}", line.set, line.oracle)),
            }
        }

        Self { cards: cards }
    }

    pub fn invert(&mut self) {
        for card in &mut self.cards {
            card.quantity *= -1;
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OwnedCard {
    pub pid: String,
    pub var: Vec<String>,
}
pub struct Collection {
    pub cards: Vec<(u32, OwnedCard)>,
}

impl Collection {
    pub fn new() -> Self {
        Collection { cards: vec![] }
    }

    pub fn merge(&mut self, patch: &Pile) {
        for new in &patch.cards {
            let mut found = false;
            for (i, owned) in self.cards.iter().enumerate() {
                if owned.1.pid == new.id {
                    // FIXME ignores variants
                    found = true;
                    self.cards[i].0 = match new.quantity + self.cards[i].0 as i32 {
                        q if q < 0 => 0,
                        q => q as u32,
                    };
                    if self.cards[i].0 == 0 {
                        self.cards.remove(i);
                    }
                    break;
                }
            }
            if !found && new.quantity > 0 {
                self.cards.push((
                    new.quantity as u32,
                    OwnedCard {
                        pid: new.id.to_string(),
                        var: vec![], // FIXME ignores variants
                    },
                ));
            }
        }
    }
}

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
        for (_, set) in scryfall::sets(root) {
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

    pub fn add_set(&mut self, set: &scryfall::Set) {
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

impl std::convert::From<&scryfall::Set> for Set {
    fn from(set: &scryfall::Set) -> Self {
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
    pub power: String,
    pub tough: String,

    pub legal: Legality,

    pub color_identity: Vec<String>,
    pub colors: Vec<String>,
}

impl std::convert::From<&scryfall::Card> for OracleCard {
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
            power: match &card.power {
                Some(s) => s.to_string(),
                None => "".to_string(),
            },
            tough: match &card.toughness {
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

    pub illustration: String,
    pub artist: String,
    pub number: String,
    pub flavor: String,

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

impl std::convert::From<&scryfall::Card> for PrintCard {
    fn from(card: &scryfall::Card) -> Self {
        PrintCard {
            id: card.id.to_string(),
            oid: card.oracle_id.to_string(),
            artist: card.artist.to_string(),
            number: card.collector_number.to_string(),
            frame: card.frame.to_string(),
            border: card.border_color.to_string(),
            layout: card.layout.to_string(),

            illustration: match &card.illustration_id {
                Some(s) => s.to_string(),
                None => card.id.to_string(),
            },

            flavor: match &card.flavor_text {
                Some(s) => s.to_string(),
                None => "".to_string(),
            },

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

#[cfg(test)]
mod test {
    use serde_json::json;
    use crate::prelude::*;
    use crate::cdif;
    use super::*;

    #[test]
    fn should_be_able_to_convert_cdif_to_a_pile() {
        let map = Map::from_file("test/lookup.json").expect("reading lookup map");
        assert_eq!(
            map.get("MIR Barbed-Back Wurm"),
            Some(&"1b96810d-72d3-4dee-a29f-cdf85ea5ce6f".to_string())
        );

        let f = cdif::File::from_file("test/cdif/mirage-wurm.cdif").unwrap();
        let pile = Pile::resolve(f, map);
        assert_eq!(pile.cards.len(), 1);

        let card = &pile.cards[0];
        assert_eq!(card.quantity, 4);
        assert_eq!(card.id, "1b96810d-72d3-4dee-a29f-cdf85ea5ce6f");
        assert_eq!(card.gvars.len(), 0);
        assert_eq!(card.lvars.len(), 0);
    }

    #[test]
    fn should_be_able_to_invert_a_cdif_pile() {
        let map = Map::from_file("test/lookup.json").expect("reading lookup map");
        assert_eq!(
            map.get("MIR Barbed-Back Wurm"),
            Some(&"1b96810d-72d3-4dee-a29f-cdf85ea5ce6f".to_string())
        );

        let f = cdif::File::from_file("test/cdif/mirage-wurm.cdif").unwrap();
        let mut pile = Pile::resolve(f, map);
        pile.invert();
        assert_eq!(pile.cards.len(), 1);

        let card = &pile.cards[0];
        assert_eq!(card.quantity, -4);
        assert_eq!(card.id, "1b96810d-72d3-4dee-a29f-cdf85ea5ce6f");
        assert_eq!(card.gvars.len(), 0);
        assert_eq!(card.lvars.len(), 0);
    }

    fn owned(qty: u32, pid: &str, var: Option<Vec<String>>) -> (u32, OwnedCard) {
        (qty, OwnedCard{
            pid: pid.to_string(),
            var: match var {
                Some(v) => v,
                None => vec![],
            },
        })
    }

    #[test]
    fn should_serialize_a_collection_into_json() {
        let mut c = Collection::new();
        c.cards.push(owned(22, "mir-plains", None));
        c.cards.push(owned(26, "mir-island", None));
        c.cards.push(owned(21, "mir-swamp", None));
        c.cards.push(owned(24, "mir-mountain", None));
        c.cards.push(owned(23, "mir-forest", None));

        assert_eq!(
            json!(c.cards).to_string(),
            r#"[[22,{"pid":"mir-plains","var":[]}],[26,{"pid":"mir-island","var":[]}],[21,{"pid":"mir-swamp","var":[]}],[24,{"pid":"mir-mountain","var":[]}],[23,{"pid":"mir-forest","var":[]}]]"#
        );
    }

    #[test]
    fn should_allow_merging_to_change_quantity_of_owned_cards() {
        let mut c = Collection::new();
        c.cards.push(owned(1, "mir-plains", None));

        c.merge(&Pile {
            cards: vec![Card {
                id: "mir-plains".to_string(),
                quantity: 2,
                gvars: vec![],
                lvars: vec![],
            }],
        });

        assert_eq!(json!(c.cards).to_string(), r#"[[3,{"pid":"mir-plains","var":[]}]]"#);
    }

    #[test]
    fn should_allow_merging_to_introduce_new_owned_cards() {
        let mut c = Collection::new();
        c.cards.push(owned(1, "mir-plains", None));

        c.merge(&Pile {
            cards: vec![Card {
                id: "mir-swamp".to_string(),
                quantity: 2,
                gvars: vec![],
                lvars: vec![],
            }],
        });

        assert_eq!(
            json!(c.cards).to_string(),
            r#"[[1,{"pid":"mir-plains","var":[]}],[2,{"pid":"mir-swamp","var":[]}]]"#
        );
    }

    #[test]
    fn should_allow_merging_to_remove_cards_no_longer_owned() {
        let mut c = Collection::new();
        c.cards.push(owned(1, "mir-plains", None));

        c.merge(&Pile {
            cards: vec![Card {
                id: "mir-plains".to_string(),
                quantity: -1,
                gvars: vec![],
                lvars: vec![],
            }],
        });

        assert_eq!(json!(c.cards).to_string(), r#"[]"#);
    }

    #[test]
    fn should_allow_merging_to_remove_cards_in_multiple_passes() {
        let mut c = Collection::new();
        c.cards.push(owned(5, "mir-plains", None));

        c.merge(&Pile {
            cards: vec![
                Card {
                    id: "mir-plains".to_string(),
                    quantity: -3,
                    gvars: vec![],
                    lvars: vec![],
                },
                Card {
                    id: "mir-plains".to_string(),
                    quantity: -2,
                    gvars: vec![],
                    lvars: vec![],
                },
            ],
        });

        assert_eq!(json!(c.cards).to_string(), r#"[]"#);
    }

    #[test]
    fn should_allow_merging_to_remove_cards_with_extreme_prejudice() {
        let mut c = Collection::new();
        c.cards.push(owned(1, "mir-plains", None));

        c.merge(&Pile {
            cards: vec![
                Card {
                    id: "mir-plains".to_string(),
                    quantity: -3,
                    gvars: vec![],
                    lvars: vec![],
                },
                Card {
                    id: "mir-plains".to_string(),
                    quantity: -2,
                    gvars: vec![],
                    lvars: vec![],
                },
            ],
        });

        assert_eq!(json!(c.cards).to_string(), r#"[]"#);
    }

    #[test]
    fn should_merge_without_getting_tripped_up_by_mid_iter_removal() {
        let mut c = Collection::new();
        c.cards.push(owned(1, "mir-plains", None));
        c.cards.push(owned(3, "mir-swamp", None));

        c.merge(&Pile {
            cards: vec![
                Card {
                    id: "mir-plains".to_string(),
                    quantity: -1,
                    gvars: vec![],
                    lvars: vec![],
                },
                Card {
                    id: "mir-swamp".to_string(),
                    quantity: -1,
                    gvars: vec![],
                    lvars: vec![],
                },
            ],
        });

        let s = json!(c.cards).to_string();
        assert_eq!(s, r#"[[2,{"pid":"mir-swamp","var":[]}]]"#);
    }
}
