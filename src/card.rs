use serde::{Serialize, Deserialize};

use crate::data::{cdif, pool};

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
    pub fn resolve(file: cdif::File, lookup: pool::Map) -> Self {
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

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use super::Pile;
    use crate::data::{pool, cdif};

    #[test]
    fn should_be_able_to_convert_cdif_to_a_pile() {
        let map = pool::Map::from_file("test/lookup.json").expect("reading lookup map");
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
        let map = pool::Map::from_file("test/lookup.json").expect("reading lookup map");
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
}
