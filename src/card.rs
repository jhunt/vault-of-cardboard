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

#[cfg(test)]
mod test {
    use serde_json::json;
    use crate::prelude::*;
    use crate::data::{pool, cdif};
    use super::*;

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
