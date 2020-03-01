#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate error_chain;

pub mod api;
pub mod card;
pub mod cdif;
pub mod db;
pub mod schema;
pub mod scryfall;
pub mod prelude;

#[cfg(test)]
mod test {
    use crate::{card, scryfall};

    #[test]
    pub fn it_should_parse_raw_scryfall_set_files() {
        let sets = scryfall::sets("test/scryfall/sets");
        assert!(sets.contains_key("5f06acf3-8123-4a78-b2e7-089b0b775a4a"));

        let mir = &sets["5f06acf3-8123-4a78-b2e7-089b0b775a4a"];

        let set = card::Set::from(mir);
        assert_eq!(set.code, "MIR");
        assert_eq!(set.name, "Mirage");
        assert_eq!(set.released_at, "1996-10-08");
        assert_eq!(350, set.cards.len());

        let card = &set.cards[0];
        assert_eq!("4644694d-52e6-4d00-8cad-748899eeea84", card.id);
        assert_eq!("4c13e2b5-961a-4031-84b1-15bd19b94286", card.oid);
        assert_eq!("Pete Venters", card.artist);
        assert_eq!("1", card.number);
        assert_eq!("1997", card.frame);
        assert_eq!("black", card.border);
        assert_eq!("normal", card.layout);
        assert_eq!("2", card.flags.pack()); // uncommon
    }

    #[test]
    pub fn it_should_parse_oracle_cards_into_a_pool() {
        let mut pool = card::Pool::new();
        for (_, set) in scryfall::sets("test/scryfall/sets") {
            pool.add_set(&set);
        }
        assert!(pool.cards.contains_key("4c13e2b5-961a-4031-84b1-15bd19b94286"));
        let card = &pool.cards["4c13e2b5-961a-4031-84b1-15bd19b94286"];

        assert_eq!("4c13e2b5-961a-4031-84b1-15bd19b94286", card.id);
        assert_eq!("Afterlife", card.name);
        assert_eq!("Instant", card.type_line);
        assert_eq!("Destroy target creature. It can't be regenerated. Its controller creates a 1/1 white Spirit creature token with flying.", card.text);
        assert_eq!(3.0, card.cmc);
        assert_eq!("{2}{W}", card.mana_cost);
        assert_eq!("EdlB$v", card.legal.pack());
        assert_eq!(vec!["W"], card.color_identity);
        assert_eq!(vec!["W"], card.colors);
    }
}
