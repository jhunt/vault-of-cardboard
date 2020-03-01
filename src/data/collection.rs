use serde::{Serialize, Deserialize};
use super::pile;

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub pid: String,
    pub var: Vec<String>,
}
pub type OwnedCard = (u32, Card);

pub struct Collection {
    pub cards: Vec<OwnedCard>,
}

impl Collection {
    pub fn new() -> Self {
        Collection { cards: vec![] }
    }

    pub fn merge(&mut self, patch: &pile::Pile) {
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
                    Card {
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
    use super::*;
    use serde_json::json;

    #[test]
    fn should_serialize_a_collection_into_json() {
        let mut c = Collection::new();
        c.cards.push((
            22,
            Card {
                pid: "mir-plains".to_string(),
                var: vec![],
            },
        ));
        c.cards.push((
            26,
            Card {
                pid: "mir-island".to_string(),
                var: vec![],
            },
        ));
        c.cards.push((
            21,
            Card {
                pid: "mir-swamp".to_string(),
                var: vec![],
            },
        ));
        c.cards.push((
            24,
            Card {
                pid: "mir-mountain".to_string(),
                var: vec![],
            },
        ));
        c.cards.push((
            23,
            Card {
                pid: "mir-forest".to_string(),
                var: vec![],
            },
        ));

        assert_eq!(
            json!(c.cards).to_string(),
            r#"[[22,{"pid":"mir-plains","var":[]}],[26,{"pid":"mir-island","var":[]}],[21,{"pid":"mir-swamp","var":[]}],[24,{"pid":"mir-mountain","var":[]}],[23,{"pid":"mir-forest","var":[]}]]"#
        );
    }

    #[test]
    fn should_allow_merging_to_change_quantity_of_owned_cards() {
        use super::pile;

        let mut c = Collection::new();
        c.cards.push((
            1,
            Card {
                pid: "mir-plains".to_string(),
                var: vec![],
            },
        ));

        c.merge(&pile::Pile {
            cards: vec![pile::Card {
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
        use super::pile;

        let mut c = Collection::new();
        c.cards.push((
            1,
            Card {
                pid: "mir-plains".to_string(),
                var: vec![],
            },
        ));

        c.merge(&pile::Pile {
            cards: vec![pile::Card {
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
        use super::pile;

        let mut c = Collection::new();
        c.cards.push((
            1,
            Card {
                pid: "mir-plains".to_string(),
                var: vec![],
            },
        ));

        c.merge(&pile::Pile {
            cards: vec![pile::Card {
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
        use super::pile;

        let mut c = Collection::new();
        c.cards.push((
            5,
            Card {
                pid: "mir-plains".to_string(),
                var: vec![],
            },
        ));

        c.merge(&pile::Pile {
            cards: vec![
                pile::Card {
                    id: "mir-plains".to_string(),
                    quantity: -3,
                    gvars: vec![],
                    lvars: vec![],
                },
                pile::Card {
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
        use super::pile;

        let mut c = Collection::new();
        c.cards.push((
            1, // we're going to "lose" 5 of them...
            Card {
                pid: "mir-plains".to_string(),
                var: vec![],
            },
        ));

        c.merge(&pile::Pile {
            cards: vec![
                pile::Card {
                    id: "mir-plains".to_string(),
                    quantity: -3,
                    gvars: vec![],
                    lvars: vec![],
                },
                pile::Card {
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
        use super::pile;

        let mut c = Collection::new();
        c.cards.push((
            1,
            Card {
                pid: "mir-plains".to_string(),
                var: vec![],
            },
        ));
        c.cards.push((
            3,
            Card {
                pid: "mir-swamp".to_string(),
                var: vec![],
            },
        ));

        c.merge(&pile::Pile {
            cards: vec![
                pile::Card {
                    id: "mir-plains".to_string(),
                    quantity: -1,
                    gvars: vec![],
                    lvars: vec![],
                },
                pile::Card {
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
