use serde::{Deserialize, Serialize, Serializer};
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
            encode self.pauper     => 'P',
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
        match card.layout.as_str() {
            "transform" => OracleCard {
                id: card.oracle_id.to_string(),
                name: card.name.to_string(),
                type_line: card.type_line.to_string(),
                text: match &card.card_faces {
                    Some(faces) => faces
                        .iter()
                        .filter(|f| f.oracle_text.is_some())
                        .map(|f| f.oracle_text.as_ref().unwrap().to_string())
                        .collect::<Vec<_>>()
                        .join("//")
                        .to_string(),
                    _ => "".to_string(),
                },
                cmc: card.cmc,
                mana_cost: match &card.card_faces {
                    Some(faces) if faces.len() > 1 => match &faces[0].mana_cost {
                        Some(s) => s.to_string(),
                        None => "".to_string(),
                    },
                    _ => "".to_string(),
                },
                power: match &card.card_faces {
                    Some(faces) => faces
                        .iter()
                        .filter(|f| f.power.is_some())
                        .map(|f| f.power.as_ref().unwrap().to_string())
                        .collect::<Vec<_>>()
                        .join("//")
                        .to_string(),
                    _ => "".to_string(),
                },
                tough: match &card.card_faces {
                    Some(faces) => faces
                        .iter()
                        .filter(|f| f.toughness.is_some())
                        .map(|f| f.toughness.as_ref().unwrap().to_string())
                        .collect::<Vec<_>>()
                        .join("//")
                        .to_string(),
                    _ => "".to_string(),
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
            },
            _ => OracleCard {
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

pub struct Frame {
    pub legendary: bool,
    pub miracle: bool,
    pub nyx_touched: bool,
    pub draft: bool,
    pub devoid: bool,
    pub tombstone: bool,
    pub color_shifted: bool,
    pub showcase: bool,
    pub compass: bool,
    pub extended_art: bool,
    pub companion: bool,

    pub frame: String,
}

impl Frame {
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
            encode self.legendary       => 'L',
            encode self.miracle         => 'm',
            encode self.nyx_touched     => 'n',
            encode self.draft           => 'D',
            encode self.devoid          => 'd',
            encode self.tombstone       => 't',
            encode self.color_shifted   => '$',
            encode self.showcase        => 's',
            encode self.compass         => 'c',
            encode self.extended_art    => '+',
            encode self.companion       => 'C'
        };

        match self.frame.as_str() {
            "1993" => s.push('3'),
            "1997" => s.push('7'),
            "2003" => s.push('M'),
            "2015" => s.push('N'),
            "future" => s.push('F'),
            _ => (),
        };

        s
    }
}

impl Serialize for Frame {
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

    pub border: String,
    pub layout: String,

    pub flags: Flags,
    pub frame: Frame,
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

fn maybe_frame(s: &'static str, effects: &Option<Vec<String>>) -> bool {
    match effects {
        Some(effects) => {
            for effect in effects {
                if s == effect {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}

impl std::convert::From<&scryfall::Card> for PrintCard {
    fn from(card: &scryfall::Card) -> Self {
        let frame = Frame {
            frame: card.frame.to_string(),
            legendary: maybe_frame("legendary", &card.frame_effects),
            miracle: maybe_frame("miracle", &card.frame_effects),
            nyx_touched: maybe_frame("nyxtouched", &card.frame_effects),
            draft: maybe_frame("draft", &card.frame_effects),
            devoid: maybe_frame("devoid", &card.frame_effects),
            tombstone: maybe_frame("tombstone", &card.frame_effects),
            color_shifted: maybe_frame("colorshifted", &card.frame_effects),
            showcase: maybe_frame("showcase", &card.frame_effects),
            compass: maybe_frame("compasslanddfc", &card.frame_effects),
            extended_art: maybe_frame("extendedart", &card.frame_effects),
            companion: maybe_frame("companion", &card.frame_effects),
        };

        let flags = Flags {
            full_art: maybe_true(&card.full_art),
            oversized: maybe_true(&card.oversized),
            reprint: maybe_true(&card.reprint),
            reserved: maybe_true(&card.reserved),
            variation: maybe_true(&card.variation),
            story_spotlight: maybe_true(&card.story_spotlight),
            rarity: card.rarity.to_string(),
        };
        match card.layout.as_str() {
            "transform" => PrintCard {
                id: card.id.to_string(),
                oid: card.oracle_id.to_string(),
                artist: card.artist.to_string(),
                number: card.collector_number.to_string(),
                frame: frame,
                border: card.border_color.to_string(),
                layout: card.layout.to_string(),

                illustration: match &card.illustration_id {
                    Some(s) => s.to_string(),
                    None => card.id.to_string(),
                },

                flavor: match &card.card_faces {
                    Some(faces) => faces
                        .iter()
                        .filter(|f| f.flavor_text.is_some())
                        .map(|f| f.flavor_text.as_ref().unwrap().to_string())
                        .collect::<Vec<_>>()
                        .join("//")
                        .to_string(),
                    _ => "".to_string(),
                },

                flags: flags,
            },
            _ => PrintCard {
                id: card.id.to_string(),
                oid: card.oracle_id.to_string(),
                artist: card.artist.to_string(),
                number: card.collector_number.to_string(),
                frame: frame,
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

                flags: flags,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cdif;
    use serde_json::json;

    #[test]
    fn should_be_able_to_pack_flags() {
        let new = || -> Flags {
            Flags {
                full_art: false,
                oversized: false,
                reprint: false,
                reserved: false,
                variation: false,
                story_spotlight: false,
                rarity: "".to_string(),
            }
        };
        assert_eq!(new().pack(), "", "the empty Flags packs to an empty string");

        let mut flags = new();
        flags.rarity = "common".to_string();
        assert_eq!(flags.pack(), "1", "commons should encode to '1'");

        let mut flags = new();
        flags.rarity = "uncommon".to_string();
        assert_eq!(flags.pack(), "2", "uncommons should encode to '2'");

        let mut flags = new();
        flags.rarity = "rare".to_string();
        assert_eq!(flags.pack(), "3", "rares should encode to '3'");

        let mut flags = new();
        flags.rarity = "mythic".to_string();
        assert_eq!(flags.pack(), "4", "commons should encode to '4'");

        let mut flags = new();
        flags.full_art = true;
        assert_eq!(flags.pack(), "^", "'full art' is packed to '^'");

        let mut flags = new();
        flags.reprint = true;
        assert_eq!(flags.pack(), "+", "'reprint' is packed to '+'");

        let mut flags = new();
        flags.reserved = true;
        assert_eq!(flags.pack(), "!", "'reserved' is packed to '!'");

        let mut flags = new();
        flags.variation = true;
        assert_eq!(flags.pack(), "~", "'variation' is packed to '~'");

        let mut flags = new();
        flags.story_spotlight = true;
        assert_eq!(flags.pack(), "@", "'story spotlight' is packed to '@'");

        let mut flags = new();
        flags.rarity = "rare".to_string();
        flags.full_art = true;
        flags.reprint = true;
        flags.reserved = true;
        flags.variation = true;
        flags.story_spotlight = true;
        assert_eq!(flags.pack(), "^+!~@3", "all flags can coexist");

        let mut flags = new();
        flags.rarity = "mythic".to_string();
        flags.variation = true;
        flags.story_spotlight = true;
        assert_eq!(flags.pack(), "~@4", "subsets of flags can coexist");
    }

    #[test]
    fn should_be_able_to_pack_legality() {
        let new = |on: bool| -> Legality {
            Legality {
                brawl: on,
                commander: on,
                duel: on,
                frontier: on,
                future: on,
                historic: on,
                legacy: on,
                modern: on,
                old_school: on,
                pauper: on,
                penny: on,
                pioneer: on,
                standard: on,
                vintage: on,
            }
        };

        assert_eq!(
            new(false).pack(),
            "",
            "the empty Legality packs to the empty string"
        );

        assert_eq!(
            new(true).pack(),
            "BEdjfhlmoP$psv",
            "the full Legality packs appropriately"
        );

        let mut legal = new(false);
        legal.brawl = true;
        assert_eq!(legal.pack(), "B", "'brawl' is packed to 'B'");

        let mut legal = new(false);
        legal.commander = true;
        assert_eq!(legal.pack(), "E", "'commander' is packed to 'E'");

        let mut legal = new(false);
        legal.duel = true;
        assert_eq!(legal.pack(), "d", "'duel' is packed to 'd'");

        let mut legal = new(false);
        legal.frontier = true;
        assert_eq!(legal.pack(), "j", "'frontier' is packed to 'j'");

        let mut legal = new(false);
        legal.future = true;
        assert_eq!(legal.pack(), "f", "'future' is packed to 'f'");

        let mut legal = new(false);
        legal.historic = true;
        assert_eq!(legal.pack(), "h", "'historic' is packed to 'h'");

        let mut legal = new(false);
        legal.legacy = true;
        assert_eq!(legal.pack(), "l", "'legacy' is packed to 'l'");

        let mut legal = new(false);
        legal.modern = true;
        assert_eq!(legal.pack(), "m", "'modern' is packed to 'm'");

        let mut legal = new(false);
        legal.old_school = true;
        assert_eq!(legal.pack(), "o", "'old_school' is packed to 'o'");

        let mut legal = new(false);
        legal.pauper = true;
        assert_eq!(legal.pack(), "P", "'pauper' is packed to 'P'");

        let mut legal = new(false);
        legal.penny = true;
        assert_eq!(legal.pack(), "$", "'penny' is packed to '$'");

        let mut legal = new(false);
        legal.pioneer = true;
        assert_eq!(legal.pack(), "p", "'pioneer' is packed to 'p'");

        let mut legal = new(false);
        legal.standard = true;
        assert_eq!(legal.pack(), "s", "'standard' is packed to 's'");

        let mut legal = new(false);
        legal.vintage = true;
        assert_eq!(legal.pack(), "v", "'vintage' is packed to 'v'");

        let mut legal = new(true);
        legal.vintage = false;
        legal.modern = false;
        assert_eq!(
            legal.pack(),
            "BEdjfhloP$ps",
            "subsets of legalities can be packed"
        );
    }

    #[test]
    fn should_be_able_to_pack_frames() {
        let new = |on: bool| -> Frame {
            Frame {
                frame: "".to_string(),

                legendary: on,
                miracle: on,
                nyx_touched: on,
                draft: on,
                devoid: on,
                tombstone: on,
                color_shifted: on,
                showcase: on,
                compass: on,
                extended_art: on,
                companion: on,
            }
        };

        assert_eq!(
            new(false).pack(),
            "",
            "the empty Frame packs to the empty string"
        );
        assert_eq!(
            new(true).pack(),
            "LmnDdt$sc+C",
            "the full Frame packs appropriately"
        );

        let mut frame = new(false);
        frame.frame = "1993".to_string();
        assert_eq!(frame.pack(), "3", "'1993' should pack to '3'");

        let mut frame = new(false);
        frame.frame = "1997".to_string();
        assert_eq!(frame.pack(), "7", "'1997' should pack to '7'");

        let mut frame = new(false);
        frame.frame = "2003".to_string();
        assert_eq!(frame.pack(), "M", "'modern' should pack to 'M'");

        frame.frame = "2015".to_string();
        assert_eq!(frame.pack(), "N", "'2015' should back to 'N'");

        let mut frame = new(false);
        frame.frame = "future".to_string();
        assert_eq!(frame.pack(), "F", "'future' should pack to 'F'");

        let mut frame = new(false);
        frame.legendary = true;
        assert_eq!(frame.pack(), "L", "'legendary' should pack to 'L'");

        let mut frame = new(false);
        frame.miracle = true;
        assert_eq!(frame.pack(), "m", "'miracle' should pack to 'm'");

        let mut frame = new(false);
        frame.nyx_touched = true;
        assert_eq!(frame.pack(), "n", "'nyx-touched' should pack to 'n'");

        let mut frame = new(false);
        frame.draft = true;
        assert_eq!(frame.pack(), "D", "'draft' should pack to 'D'");

        let mut frame = new(false);
        frame.devoid = true;
        assert_eq!(frame.pack(), "d", "'devoid' should pack to 'd'");

        let mut frame = new(false);
        frame.tombstone = true;
        assert_eq!(frame.pack(), "t", "'tombstone' should pack to 't'");

        let mut frame = new(false);
        frame.color_shifted = true;
        assert_eq!(frame.pack(), "$", "'color-shifted' should pack to '$'");

        let mut frame = new(false);
        frame.showcase = true;
        assert_eq!(frame.pack(), "s", "'showcase' should pack to 's'");

        let mut frame = new(false);
        frame.compass = true;
        assert_eq!(frame.pack(), "c", "'compass' should pack to 'c'");

        let mut frame = new(false);
        frame.extended_art = true;
        assert_eq!(frame.pack(), "+", "'extended-art' should pack to '+'");

        let mut frame = new(false);
        frame.companion = true;
        assert_eq!(frame.pack(), "C", "'companion' should pack to 'C'");

        let mut frame = new(false);
        frame.frame = "2003".to_string();
        frame.legendary = true;
        frame.companion = true;
        assert_eq!(frame.pack(), "LCM", "subsets of frames can coexist");
    }

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
        (
            qty,
            OwnedCard {
                pid: pid.to_string(),
                var: match var {
                    Some(v) => v,
                    None => vec![],
                },
            },
        )
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

        assert_eq!(
            json!(c.cards).to_string(),
            r#"[[3,{"pid":"mir-plains","var":[]}]]"#
        );
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
