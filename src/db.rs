use super::schema::{collections, collectors, decks, transactions};
use chrono::{naive::NaiveDate, DateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(std::fmt::Debug)]
pub struct Error {}

impl std::convert::From<uuid::parser::ParseError> for Error {
    fn from(_e: uuid::parser::ParseError) -> Error {
        Error {} // FIXME
    }
}

impl std::convert::From<diesel::result::Error> for Error {
    fn from(_e: diesel::result::Error) -> Error {
        Error {} // FIXME
    }
}

impl std::convert::From<diesel::ConnectionError> for Error {
    fn from(_e: diesel::ConnectionError) -> Error {
        Error {} // FIXME
    }
}

impl std::convert::From<diesel_migrations::RunMigrationsError> for Error {
    fn from(_e: diesel_migrations::RunMigrationsError) -> Error {
        Error {} // FIXME
    }
}

#[derive(Identifiable, Queryable)]
pub struct Collector {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "collectors"]
pub struct NewCollector<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(AsChangeset)]
#[table_name = "collectors"]
pub struct CollectorUpdate<'a> {
    pub username: Option<&'a str>,
    pub email: Option<&'a str>,
    pub password: Option<&'a str>,
}

#[derive(Queryable)]
pub struct Collection {
    pub id: Uuid,
    pub collector: Uuid,
}

#[derive(Insertable)]
#[table_name = "collections"]
pub struct NewCollection {
    pub id: Uuid,
    pub collector: Uuid,
}

#[derive(Queryable)]
pub struct Transaction {
    pub id: Uuid,
    pub collection: Uuid,
    pub dated: NaiveDate,
    pub gain: String,
    pub loss: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction<'a> {
    pub collection: Uuid,
    pub dated: &'a NaiveDate,
    pub gain: &'a str,
    pub loss: &'a str,
}

#[derive(Queryable)]
pub struct Deck {
    pub id: Uuid,
    pub collector: Uuid,
    pub title: String,
    pub description: String,
    pub main: String,
    pub side: String,
    pub maybe: String,
    pub lineage: Uuid,
    pub ordinal: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "decks"]
pub struct NewDeck<'a> {
    pub collector: Uuid,
    pub title: &'a str,
    pub description: &'a str,
    pub main: &'a str,
    pub side: &'a str,
    pub maybe: &'a str,
}

pub struct Database {
    conn: PgConnection,
}

embed_migrations!("migrations/");

// Generate a random UUID, if one wasn't supplied.
fn gen_uuid(id: Option<Uuid>) -> Uuid {
    match id {
        Some(id) => id,
        None => Uuid::new_v4(),
    }
}

impl Database {
    // Connect to a DSN (must be PostgreSQL) and run migrations.
    pub fn connect(dsn: &str) -> Result<Database, Error> {
        let db = Database {
            conn: PgConnection::establish(dsn)?,
        };

        embedded_migrations::run(&db.conn)?;
        Ok(db)
    }

    #[cfg(test)]
    // Start a transaction that will never end.
    //
    // This is only available in test runs, and is meant solely for being
    // able to run SQL queries / statements until an error is encountered,
    // with implicit (but unavoidable) transactional rollback.
    //
    pub fn testmode(&self) -> Result<(), Error> {
        self.conn.begin_test_transaction()?;
        Ok(())
    }

    // Insert a new Collector, and their 1:1 Collection.
    //
    // The caller may supply an explicit UUID as the first parameter, or
    // None to automatically generate a v4 UUID.
    //
    pub fn create_collector(
        &self,
        id: Option<Uuid>,
        new: NewCollector,
    ) -> Result<Collector, Error> {
        let id = gen_uuid(id);
        let collector = diesel::insert_into(collectors::table)
            .values((&new, collectors::dsl::id.eq(id)))
            .get_result(&self.conn)?;

        diesel::insert_into(collections::table)
            .values(&NewCollection { id, collector: id })
            .get_result::<Collection>(&self.conn)?;

        Ok(collector)
    }

    // Find a Collector by their UUID.
    pub fn find_collector_by_uuid(&self, id: Uuid) -> Result<Option<Collector>, Error> {
        Ok(Some(
            collectors::dsl::collectors
                .find(id)
                .get_result::<Collector>(&self.conn)?,
        ))
    }

    // Update a Collector with a blanket patch
    //
    // The CollectorUpdate object is just a collection of Option<...>
    // fields, to make it possible to do partial updates flexibly.
    //
    // Returns the final Collector object, after updates are applied.
    //
    pub fn update_collector(
        &self,
        obj: &Collector,
        upd: CollectorUpdate,
    ) -> Result<Collector, Error> {
        Ok(diesel::update(obj).set(&upd).get_result(&self.conn)?)
    }

    // Find a Collection by its UUID.
    pub fn find_collection_by_uuid(&self, id: Uuid) -> Result<Option<Collection>, Error> {
        Ok(Some(
            collections::dsl::collections
                .find(id)
                .get_result::<Collection>(&self.conn)?,
        ))
    }

    // Create a new Transaction.
    pub fn create_transaction(
        &self,
        id: Option<Uuid>,
        new: NewTransaction,
    ) -> Result<Transaction, Error> {
        Ok(diesel::insert_into(transactions::table)
            .values((&new, transactions::dsl::id.eq(gen_uuid(id))))
            .get_result(&self.conn)?)
    }

    //pub fn update_transaction
    //pub fn delete_transaction

    pub fn create_deck(&self, id: Option<Uuid>, new: NewDeck) -> Result<Deck, Error> {
        let now = Utc::now();
        let id = gen_uuid(id);
        Ok(diesel::insert_into(decks::table)
            .values((
                &new,
                decks::dsl::id.eq(id),
                decks::dsl::lineage.eq(id),
                decks::dsl::ordinal.eq(0),
                decks::dsl::created_at.eq(now),
                decks::dsl::updated_at.eq(now),
            ))
            .get_result(&self.conn)?)
    }

    //pub fn snapshot_deck
    //pub fn update_deck
    //pub fn delete_deck
}

#[cfg(test)]
mod test {
    use super::*;
    use uuid::Uuid;

    fn connect() -> Database {
        use std::env;

        let dsn = env::var("TEST_DATABASE_URL")
            .or_else(|_| env::var("DATABASE_URL"))
            .expect("Either TEST_DATABASE_URL or DATABASE_URL must be set in the environment.");
        let db = Database::connect(&dsn).unwrap();
        db.testmode().unwrap();
        db
    }

    #[test]
    pub fn it_should_be_able_to_create_a_collector() {
        let db = connect();

        let id = Uuid::parse_str("a026089c-90a2-4180-b41a-2082e7b2ebcc").unwrap();
        let jhunt = db.create_collector(
            Some(id),
            NewCollector {
                username: "jhunt",
                email: "james@example.com",
                password: "sekrit",
            },
        );
        assert!(jhunt.is_ok());

        let found = db.find_collector_by_uuid(id);
        assert!(found.is_ok());
        let found = found.unwrap();
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.username, "jhunt");
        assert_eq!(found.email, "james@example.com");
    }

    #[test]
    pub fn it_should_not_be_able_to_reuse_usernames() {
        let db = connect();

        let jhunt = db.create_collector(
            None,
            NewCollector {
                username: "jhunt",
                email: "james@example.com",
                password: "sekrit",
            },
        );
        assert!(jhunt.is_ok());
        let jhunt = jhunt.unwrap();

        let found = db.find_collector_by_uuid(jhunt.id);
        assert!(found.is_ok());
        let found = found.unwrap();
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.username, "jhunt");
        assert_eq!(found.email, "james@example.com");

        let err = db.create_collector(
            None,
            NewCollector {
                username: &found.username,
                email: "other-james@example.com",
                password: "a different sekrit",
            },
        );
        assert!(err.is_err());
    }

    #[test]
    pub fn it_should_be_able_to_reuse_email_addresses() {
        let db = connect();

        let jhunt = db.create_collector(
            None,
            NewCollector {
                username: "jhunt",
                email: "james@example.com",
                password: "sekrit",
            },
        );
        assert!(jhunt.is_ok());
        let jhunt = jhunt.unwrap();

        let found = db.find_collector_by_uuid(jhunt.id);
        assert!(found.is_ok());
        let found = found.unwrap();
        assert!(found.is_some());
        let found = found.unwrap();

        let other = db.create_collector(
            None,
            NewCollector {
                username: "other-jhunt",
                email: &found.email,
                password: "a different sekrit",
            },
        );
        assert!(!other.is_err());
    }

    #[test]
    pub fn it_should_be_able_to_reuse_passwords() {
        let db = connect();

        let jhunt = db.create_collector(
            None,
            NewCollector {
                username: "jhunt",
                email: "james@example.com",
                password: "sekrit",
            },
        ).unwrap();

        db.find_collector_by_uuid(jhunt.id).unwrap();
        let other = db.create_collector(
            None,
            NewCollector {
                username: "other-jhunt",
                email: "other-james@example.com",
                password: "sekrit",
            },
        );
        assert!(!other.is_err());
    }

    #[test]
    pub fn it_should_create_a_collection_for_a_new_collector() {
        let db = connect();

        let jhunt = db.create_collector(
            None,
            NewCollector {
                username: "jhunt",
                email: "james@example.com",
                password: "sekrit",
            },
        );
        assert!(jhunt.is_ok());
        let jhunt = jhunt.unwrap();

        let collection = db.find_collection_by_uuid(jhunt.id);
        assert!(collection.is_ok());
        let collection = collection.unwrap();
        assert!(collection.is_some());
    }

    #[test]
    pub fn it_can_create_a_transaction() {
        let db = connect();

        let jhunt = db.create_collector(
            None,
            NewCollector {
                username: "jhunt",
                email: "james@example.com",
                password: "sekrit",
            },
        ).unwrap();

        let txn = db.create_transaction(None, NewTransaction{
            collection: jhunt.id,
            dated: &NaiveDate::from_ymd(2020, 01, 14),
            gain: "1x XLN Opt\n",
            loss: "",
        });
        assert!(txn.is_ok());
        let txn = txn.unwrap();

        assert_eq!(txn.collection, jhunt.id);
        assert_eq!(txn.dated, NaiveDate::from_ymd(2020, 01, 14));
        assert_eq!(txn.gain, "1x XLN Opt\n");
        assert_eq!(txn.loss, "");
    }

    #[test]
    pub fn it_can_create_a_deck() {
        let db = connect();

        let jhunt = db.create_collector(
            None,
            NewCollector {
                username: "jhunt",
                email: "james@example.com",
                password: "sekrit",
            },
        ).unwrap();

        let deck = db.create_deck(None, NewDeck{
            collector: jhunt.id,
            title: "Niv-Mizzet",
            description: "Draw a card, 1 damage to you...",
            main: "1x GRN Niv-Mizzet\n",
            side: "1x RNA Niv-Mizzet, Parun\n",
            maybe: "1x WAR Niv-Mizzet, Reborn\n",
        });
        assert!(deck.is_ok());
        let deck = deck.unwrap();

        assert_eq!(deck.collector, jhunt.id);
        assert_eq!(deck.title, "Niv-Mizzet");
        assert_eq!(deck.description, "Draw a card, 1 damage to you...");
        assert_eq!(deck.main, "1x GRN Niv-Mizzet\n");
        assert_eq!(deck.side, "1x RNA Niv-Mizzet, Parun\n");
        assert_eq!(deck.maybe, "1x WAR Niv-Mizzet, Reborn\n");
        assert_eq!(deck.lineage, deck.id);
        assert_eq!(deck.ordinal, 0);
    }
}
