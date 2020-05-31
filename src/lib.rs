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

        // Mirage
        assert!(sets.contains_key("5f06acf3-8123-4a78-b2e7-089b0b775a4a"));
        let set = card::Set::from(&sets["5f06acf3-8123-4a78-b2e7-089b0b775a4a"]);
        assert_eq!(set.code, "MIR");
        assert_eq!(set.name, "Mirage");
        assert_eq!(set.released_at, "1996-10-08");
        assert_eq!(350, set.cards.len());

        // MIR AfterLife - 1997 frame
        let card = &set.cards[0];
        assert_eq!("4644694d-52e6-4d00-8cad-748899eeea84", card.id);
        assert_eq!("4c13e2b5-961a-4031-84b1-15bd19b94286", card.oid);
        assert_eq!("Pete Venters", card.artist);
        assert_eq!("1", card.number);
        assert_eq!("7", card.frame.pack()); // 1997 frame, no specials
        assert_eq!("black", card.border);
        assert_eq!("normal", card.layout);
        assert_eq!("2", card.flags.pack()); // uncommon
    }

    #[test]
    pub fn it_should_parse_frames_from_scryfall_data() {
        let sets = scryfall::sets("test/scryfall/sets");

        // Homelands (1993)
        assert!(sets.contains_key("5ac1f606-e682-46e9-ad0f-122a3783581b"));
        let set = card::Set::from(&sets["5ac1f606-e682-46e9-ad0f-122a3783581b"]);
        assert_eq!(set.code, "HML");
        let card = &set.cards[101]; // HML An-Havva Inn - 1993 frame
        assert_eq!("eff4531f-d19d-44af-861a-33087197d21c", card.id);
        assert_eq!("3", card.frame.pack());

        // Mirage (1997)
        assert!(sets.contains_key("5f06acf3-8123-4a78-b2e7-089b0b775a4a"));
        let set = card::Set::from(&sets["5f06acf3-8123-4a78-b2e7-089b0b775a4a"]);
        assert_eq!(set.code, "MIR");
        let card = &set.cards[0]; // Afterlife - 1997 frame
        assert_eq!("4644694d-52e6-4d00-8cad-748899eeea84", card.id);
        assert_eq!("7", card.frame.pack());

        // Core 2011 (modern)
        assert!(sets.contains_key("485d2468-18c8-42a4-9482-ca1c51e0470e"));
        let set = card::Set::from(&sets["485d2468-18c8-42a4-9482-ca1c51e0470e"]);
        assert_eq!(set.code, "M11");
        let card = &set.cards[52]; // Flashfreeze
        assert_eq!("4dc68a06-9d79-4be8-b4ce-ecd4d0e4ce29", card.id);
        assert_eq!("M", card.frame.pack());

        // Future Sight (future)
        assert!(sets.contains_key("bf951ddb-4445-4923-87cb-3fe4ac3c6b9a"));
        let set = card::Set::from(&sets["bf951ddb-4445-4923-87cb-3fe4ac3c6b9a"]);
        assert_eq!(set.code, "FUT");
        let card = &set.cards[54]; // Nix - future frame
        assert_eq!("3dab4f64-2a91-409a-b83b-45b22afd22ff", card.id);
        assert_eq!("F", card.frame.pack());


        // The remaining tests exercise the frame_effects keywords from
        // upstream, validating that we parse them correctly.


        // ELD - Flaxen Intruder (showcase)
        assert!(sets.contains_key("a90a7b2f-9dd8-4fc7-9f7d-8ea2797ec782"));
        let set = card::Set::from(&sets["a90a7b2f-9dd8-4fc7-9f7d-8ea2797ec782"]);
        assert_eq!(set.code, "ELD");
        let card = &set.cards[296];
        assert_eq!("489ae8ac-8329-49f0-a301-875f7ba37c97", card.id);
        assert_eq!("sN", card.frame.pack());

        // ELD - Castle Vantress (extended art)
        assert!(sets.contains_key("a90a7b2f-9dd8-4fc7-9f7d-8ea2797ec782"));
        let set = card::Set::from(&sets["a90a7b2f-9dd8-4fc7-9f7d-8ea2797ec782"]);
        assert_eq!(set.code, "ELD");
        let card = &set.cards[389];
        assert_eq!("4113eeed-9399-4b59-a6d9-7d40190853c5", card.id);
        assert_eq!("+N", card.frame.pack());

        // IKO - Lutri, the Spellchaser (legednary + companion)
        assert!(sets.contains_key("19feda43-15ab-427e-a0e4-148a4bf2b03a"));
        let set = card::Set::from(&sets["19feda43-15ab-427e-a0e4-148a4bf2b03a"]);
        assert_eq!(set.code, "IKO");
        let card = &set.cards[226];
        assert_eq!("fb1189c9-7842-466e-8238-1e02677d8494", card.id);
        assert_eq!("LCN", card.frame.pack());

        // PLC - Essence Warden (color shifted)
        assert!(sets.contains_key("5a1b571c-73e9-4c14-b9d4-a62507d85789"));
        let set = card::Set::from(&sets["5a1b571c-73e9-4c14-b9d4-a62507d85789"]);
        assert_eq!(set.code, "PLC");
        let card = &set.cards[144];
        assert_eq!("da2a65d6-0887-4fe8-a6e6-909208fddd90", card.id);
        assert_eq!("$M", card.frame.pack());

        // ODY - Dematerialize (tombstone)
        assert!(sets.contains_key("b0d90d2d-494a-4224-bfa0-36ce5ee281b1"));
        let set = card::Set::from(&sets["b0d90d2d-494a-4224-bfa0-36ce5ee281b1"]);
        assert_eq!(set.code, "ODY");
        let card = &set.cards[80];
        assert_eq!("04217c17-7c29-4b02-b9b6-bfa1df50d4bc", card.id);
        assert_eq!("t7", card.frame.pack());

        // BFZ - Brutal Expansion (devoid)
        assert!(sets.contains_key("91719374-7ac5-4afa-ada8-5da964dcf1d4"));
        let set = card::Set::from(&sets["91719374-7ac5-4afa-ada8-5da964dcf1d4"]);
        assert_eq!(set.code, "BFZ");
        let card = &set.cards[199];
        assert_eq!("8b0eb64b-a463-4f25-9278-340051139f0e", card.id);
        assert_eq!("dN", card.frame.pack());

        // CN2 - Echoing Boon (draft)
        assert!(sets.contains_key("ad1b8847-1905-4080-9e26-80691ea7c1ef"));
        let set = card::Set::from(&sets["ad1b8847-1905-4080-9e26-80691ea7c1ef"]);
        assert_eq!(set.code, "CN2");
        let card = &set.cards[2];
        assert_eq!("165dd3b0-0878-448e-b922-019d0ba372d0", card.id);
        assert_eq!("DN", card.frame.pack());

        // AVR - Vanishment (miracle)
        assert!(sets.contains_key("039810a9-92d7-4f2d-b2d0-ca661ac586c0"));
        let set = card::Set::from(&sets["039810a9-92d7-4f2d-b2d0-ca661ac586c0"]);
        assert_eq!(set.code, "AVR");
        let card = &set.cards[81];
        assert_eq!("dece40c1-790c-4471-a790-1d356b345603", card.id);
        assert_eq!("mM", card.frame.pack());

        // JOU - Aegis of the Gods (nyx touched)
        assert!(sets.contains_key("204d2dca-1887-4721-9558-164aa7bbeb4f"));
        let set = card::Set::from(&sets["204d2dca-1887-4721-9558-164aa7bbeb4f"]);
        assert_eq!(set.code, "JOU");
        let card = &set.cards[0];
        assert_eq!("f2b2f381-86a2-42ac-b694-dcde437d574f", card.id);
        assert_eq!("nM", card.frame.pack());

        // RIX - Azor's Gateway (compass)
        assert!(sets.contains_key("2f7e40fc-772d-4a85-bfdd-01653c41d0fa"));
        let set = card::Set::from(&sets["2f7e40fc-772d-4a85-bfdd-01653c41d0fa"]);
        assert_eq!(set.code, "RIX");
        let card = &set.cards[175];
        assert_eq!("303d51ab-b9c4-4647-950f-291daabe7b81", card.id);
        assert_eq!("cN", card.frame.pack());
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
