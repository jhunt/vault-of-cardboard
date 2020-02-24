use serde::Serialize;
use super::super::data::{pool, cdif};

#[derive(Serialize)]
pub struct Card {
    pub quantity: i32,
    pub id: String,
    pub gvars: Vec<String>,
    pub lvars: Vec<(String, String)>,
}

#[derive(Serialize)]
pub struct Pile {
    pub cards: Vec<Card>,
}

impl Pile {
    pub fn resolve(lines: Vec<cdif::Line>, lookup: pool::Map) -> Self {
        let mut cards = vec![];

        for line in lines {
            match lookup.get(&format!("{} {}", &line.set, &line.oracle)) {
                Some(id) => cards.push(Card{
                    id: id.to_string(),
                    quantity: line.quantity,
                    lvars: line.lvars,
                    gvars: line.gvars,
                }),
                None => panic!(format!("unable to find {} {}", line.set, line.oracle)),
            }
        }

        Self {
            cards: cards,
        }
    }

    pub fn diff(a: &Self, b: &Self) -> Self {
        Self {
            cards: vec![],
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    pub fn it_should_be_able_to_convert_cdif_to_a_pile() {

    }
}
