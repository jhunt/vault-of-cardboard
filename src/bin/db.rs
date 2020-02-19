use vault_of_cardboard::db::{self, Database};

fn main() {
    let database =
        Database::connect("postgres://postgres@localhost:9019").unwrap();
    let collector = database.create_collector(
        None,
        db::NewCollector {
            username: "jhunt",
            email: "james@example.com",
            password: "sekrit",
        },
    ).unwrap();

    let collector = database.update_collector(
        &collector,
        db::CollectorUpdate {
            username: None,
            password: None,
            email: Some("jhunt@example.com"),
        },
    ).unwrap();

    database.create_deck(
        None,
        db::NewDeck {
            collector: collector.id,
            title: "Niv-Mizzet",
            description: "My Niv Deck",
            main: "1x Niv-Mizzet\n",
            side: "1x Niv-Mizzet, Parun\n",
            maybe: "1x Niv-Mizzet, Reborn\n",
        },
    ).unwrap();
}
