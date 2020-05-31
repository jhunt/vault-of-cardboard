#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate hyper;

pub mod api;
pub mod card;
pub mod cdif;
pub mod db;
pub mod schema;
pub mod scryfall;
pub mod prelude;

pub mod cli;

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
    pub fn it_should_parse_double_faced_scryfall_cards() {
        let sets = scryfall::sets("test/scryfall/sets");
        assert!(sets.contains_key("5e914d7e-c1e9-446c-a33d-d093c02b2743"));

        let soi = &sets["5e914d7e-c1e9-446c-a33d-d093c02b2743"];
        let avacyn = &soi.cards[4];
        assert_eq!("ae155ee2-008f-4dc6-82bf-476be7baa224", avacyn.id);
        assert_eq!("432b37a5-d32a-4b78-91ab-860aa026b7cc", avacyn.oracle_id);
        assert_eq!("transform", avacyn.layout);

        let oracle = card::OracleCard::from(avacyn);
        assert_eq!("Archangel Avacyn // Avacyn, the Purifier", oracle.name);
        assert_eq!("Legendary Creature — Angel // Legendary Creature — Angel", oracle.type_line);
        assert_eq!(5.0, oracle.cmc);
        assert_eq!("{3}{W}{W}", oracle.mana_cost);
        assert_eq!("4//6", oracle.power);
        assert_eq!("4//5", oracle.tough);

        assert_eq!("Flash\nFlying, vigilance\nWhen Archangel Avacyn enters the battlefield, creatures you control gain indestructible until end of turn.\nWhen a non-Angel creature you control dies, transform Archangel Avacyn at the beginning of the next upkeep.//Flying\nWhen this creature transforms into Avacyn, the Purifier, it deals 3 damage to each other creature and each opponent.", oracle.text);

        let print = card::PrintCard::from(avacyn);
        assert_eq!("\"Wings that once bore hope are now stained with blood. She is our guardian no longer.\" —Grete, cathar apostate", print.flavor);
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
        assert_eq!("EdlP$v", card.legal.pack());
        assert_eq!(vec!["W"], card.color_identity);
        assert_eq!(vec!["W"], card.colors);
    }
}
