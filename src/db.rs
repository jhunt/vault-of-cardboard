use super::schema::{collections, collectors, decks, transactions};
use bcrypt;
use chrono::{naive::NaiveDate, DateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use redis;
use uuid::Uuid;

mod errors {
    error_chain! {}
}
pub use errors::Error;
use errors::*;

const PWHASH_COST: u32 = 10;

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

use std::collections::HashMap;
pub struct Session {
    pub id: Uuid,
    pub attrs: HashMap<String, String>,
}

impl Session {
    pub fn new(id: Option<Uuid>) -> Session {
        Session {
            id: match id {
                Some(id) => id,
                None => Uuid::new_v4(),
            },
            attrs: HashMap::new(),
        }
    }

    pub fn set(&mut self, k: &str, v: &str) {
        self.attrs.insert(k.to_string(), v.to_string());
    }

    pub fn get(&self, k: &str) -> String {
        self.attrs[k].to_string()
    }
}

pub struct Database {
    pg: PgConnection,
    rd: redis::Client,
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
    pub fn connect(pg: &str, rd: &str) -> Result<Database> {
        let db = Database {
            pg: PgConnection::establish(pg).chain_err(|| "unable to connect to database")?,
            rd: redis::Client::open(rd).chain_err(|| "unable to connect to session store")?,
        };

        embedded_migrations::run(&db.pg).chain_err(|| "failed to run database migrations")?;
        Ok(db)
    }

    #[cfg(test)]
    // Start a transaction that will never end.
    //
    // This is only available in test runs, and is meant solely for being
    // able to run SQL queries / statements until an error is encountered,
    // with implicit (but unavoidable) transactional rollback.
    //
    pub fn testmode(&self) -> Result<()> {
        self.pg
            .begin_test_transaction()
            .chain_err(|| "unable to initiate test transaction")?;
        Ok(())
    }

    // Persist a Session object to Redis.
    //
    // Session objects expire after 1h.  This is currently hard-coded.
    //
    pub fn set_session(&self, s: Session) -> Result<Session> {
        let key = format!("session:{}", s.id);

        let mut cmd = redis::cmd("HMSET");
        let mut cmd = cmd.arg(&key).arg("id").arg(s.id.to_string());
        for (k, v) in &s.attrs {
            cmd = cmd.arg(k).arg(v);
        }
        cmd.query::<bool>(
            &mut self
                .rd
                .get_connection()
                .chain_err(|| "unable to connect to session store")?,
        )
        .chain_err(|| "failed to insert session object into session store")?;

        let mut cmd = redis::cmd("EXPIRE");
        let cmd = cmd.arg(&key).arg("3600"); // FIXME
        cmd.query::<bool>(
            &mut self
                .rd
                .get_connection()
                .chain_err(|| "unable to connect to session store")?,
        )
        .chain_err(|| "failed to set session expiration in session store")?;

        Ok(s)
    }

    // Retrieve a Session object from Redis.
    pub fn get_session(&self, id: Uuid) -> Result<Option<Session>> {
        let key = format!("session:{}", id);
        let mut cmd = redis::cmd("HGETALL");
        let cmd = cmd.arg(key);

        let lst = cmd
            .query::<HashMap<String, String>>(
                &mut self
                    .rd
                    .get_connection()
                    .chain_err(|| "unable to connect to session store")?,
            )
            .chain_err(|| "failed to retrieve session from session store")?;
        Ok(if lst.contains_key("id") {
            Some(Session { id: id, attrs: lst })
        } else {
            None
        })
    }

    // Forcibly expire a Session from Redis.
    //
    // After this call, attempts to retrieve the Session object via
    // `get_session()` will return Ok(None), assuming no Redis errors
    // (transport, connection, etc.) are encountered.
    //
    pub fn expire_session(&self, s: &Session) -> Result<()> {
        let key = format!("session:{}", s.id);
        let mut cmd = redis::cmd("DEL");
        let cmd = cmd.arg(key);

        cmd.query::<i32>(
            &mut self
                .rd
                .get_connection()
                .chain_err(|| "unable to connect to session store")?,
        )
        .chain_err(|| "failed to delete session key from session store")?;

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
        password: Option<&str>,
    ) -> Result<Collector> {
        let id = gen_uuid(id);
        let crypted = match password {
            Some(password) => bcrypt::hash(password, PWHASH_COST)
                .chain_err(|| "failed to hash collector password")?,
            None => "password-not-specified".to_string(),
        };

        let collector = diesel::insert_into(collectors::table)
            .values((
                &new,
                collectors::dsl::id.eq(id),
                collectors::dsl::password.eq(crypted),
            ))
            .get_result(&self.pg)
            .chain_err(|| "failed to insert collector record into database")?;

        diesel::insert_into(collections::table)
            .values(&NewCollection { id, collector: id })
            .get_result::<Collection>(&self.pg)
            .chain_err(|| "failed to insert collection record for new collector into database")?;

        Ok(collector)
    }

    pub fn authenticate_collector(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Option<Collector>> {
        let mut found = collectors::dsl::collectors
            .filter(collectors::dsl::username.eq(username))
            .get_results::<Collector>(&self.pg)
            .chain_err(|| "failed to retrieve collector record from database")?;

        match found.len() {
            1 => {
                let collector = found.pop().unwrap();
                match bcrypt::verify(&password, &collector.password) {
                    Ok(true) => Ok(Some(collector)),
                    _ => Ok(None),
                }
            }
            _ => {
                let _ = bcrypt::verify(&password, "");
                Ok(None)
            }
        }
    }

    // Find a Collector by their UUID.
    pub fn find_collector_by_uuid(&self, id: Uuid) -> Result<Option<Collector>> {
        Ok(Some(
            collectors::dsl::collectors
                .find(id)
                .get_result::<Collector>(&self.pg)
                .chain_err(|| "failed to retrieve collector record from database")?,
        ))
    }

    // Update a Collector with a blanket patch
    //
    // The CollectorUpdate object is just a collection of Option<...>
    // fields, to make it possible to do partial updates flexibly.
    //
    // Returns the final Collector object, after updates are applied.
    //
    pub fn update_collector(&self, obj: &Collector, upd: CollectorUpdate) -> Result<Collector> {
        Ok(diesel::update(obj)
            .set(&upd)
            .get_result(&self.pg)
            .chain_err(|| "failed to update collector record in database")?)
    }

    // Find a Collection by its UUID.
    pub fn find_collection_by_uuid(&self, id: Uuid) -> Result<Option<Collection>> {
        Ok(Some(
            collections::dsl::collections
                .find(id)
                .get_result::<Collection>(&self.pg)
                .chain_err(|| "failed to retrieve collection from database")?,
        ))
    }

    // Create a new Transaction.
    pub fn create_transaction(&self, id: Option<Uuid>, new: NewTransaction) -> Result<Transaction> {
        Ok(diesel::insert_into(transactions::table)
            .values((&new, transactions::dsl::id.eq(gen_uuid(id))))
            .get_result(&self.pg)
            .chain_err(|| "failed to insert transaction record into database")?)
    }

    //pub fn update_transaction
    //pub fn delete_transaction

    pub fn create_deck(&self, id: Option<Uuid>, new: NewDeck) -> Result<Deck> {
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
            .get_result(&self.pg)
            .chain_err(|| "failed to insert deck record into database")?)
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

        let pg = env::var("TEST_DATABASE_URL")
            .or_else(|_| env::var("DATABASE_URL"))
            .expect("Either TEST_DATABASE_URL or DATABASE_URL must be set in the environment.");

        let rd = env::var("TEST_REDIS_URL")
            .or_else(|_| env::var("REDIS_URL"))
            .expect("Either TEST_REDIS_URL or REDIS_URL must be set in the environment.");

        let db = Database::connect(&pg, &rd).unwrap();
        db.testmode().unwrap();
        db
    }

    #[test]
    pub fn it_should_be_able_to_create_a_session() {
        let db = connect();

        let mut session = Session::new(None);
        session.set("foo", "bar");
        assert_eq!(session.get("foo"), "bar");

        let session = db.set_session(session).unwrap();
        assert_eq!(session.get("foo"), "bar");

        let session = db.get_session(session.id).unwrap().unwrap();
        assert_eq!(session.get("foo"), "bar");

        assert!(db.expire_session(&session).is_ok());
        assert!(db.get_session(session.id).unwrap().is_none());
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
            },
            None,
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
            },
            None,
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
            },
            None,
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
            },
            None,
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
            },
            None,
        );
        assert!(!other.is_err());
    }

    #[test]
    pub fn it_should_be_able_to_reuse_passwords() {
        let db = connect();

        let jhunt = db
            .create_collector(
                None,
                NewCollector {
                    username: "jhunt",
                    email: "james@example.com",
                },
                Some("sekrit"),
            )
            .unwrap();

        db.find_collector_by_uuid(jhunt.id).unwrap();
        let other = db.create_collector(
            None,
            NewCollector {
                username: "other-jhunt",
                email: "other-james@example.com",
            },
            Some("sekrit"),
        );
        assert!(!other.is_err());
    }

    #[test]
    pub fn it_should_be_able_to_update_a_collectors_username() {
        let db = connect();

        let jhunt = db
            .create_collector(
                None,
                NewCollector {
                    username: "jhunt",
                    email: "james@example.com",
                },
                None,
            )
            .unwrap();

        let found = db.find_collector_by_uuid(jhunt.id);
        assert!(found.is_ok());
        let found = found.unwrap();
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!("jhunt", found.username);

        let updated = db.update_collector(
            &jhunt,
            CollectorUpdate {
                username: Some("james"),
                email: None,
                password: None,
            },
        );
        assert!(updated.is_ok());
        let updated = updated.unwrap();
        assert_eq!("james", updated.username);

        let found = db.find_collector_by_uuid(updated.id);
        assert!(found.is_ok());
        let found = found.unwrap();
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!("james", found.username);
    }

    #[test]
    pub fn it_should_not_be_able_to_reuse_usernames_via_update() {
        let db = connect();

        let jhunt = db
            .create_collector(
                None,
                NewCollector {
                    username: "jhunt",
                    email: "james@example.com",
                },
                None,
            )
            .unwrap();
        db.find_collector_by_uuid(jhunt.id).unwrap().unwrap();

        let james = db
            .create_collector(
                None,
                NewCollector {
                    username: "james",
                    email: "other-james@example.com",
                },
                None,
            )
            .unwrap();
        db.find_collector_by_uuid(james.id).unwrap().unwrap();

        let updated = db.update_collector(
            &jhunt,
            CollectorUpdate {
                username: Some("james"),
                email: None,
                password: None,
            },
        );
        assert!(updated.is_err());
    }

    #[test]
    pub fn it_should_be_able_to_update_a_collectors_email() {
        let db = connect();

        let jhunt = db
            .create_collector(
                None,
                NewCollector {
                    username: "jhunt",
                    email: "james@example.com",
                },
                None,
            )
            .unwrap();

        let found = db.find_collector_by_uuid(jhunt.id);
        assert!(found.is_ok());
        let found = found.unwrap();
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!("jhunt", found.username);

        let updated = db.update_collector(
            &jhunt,
            CollectorUpdate {
                username: None,
                email: Some("jhunt@example.com"),
                password: None,
            },
        );
        assert!(updated.is_ok());
        let updated = updated.unwrap();
        assert_eq!("jhunt@example.com", updated.email);

        let found = db.find_collector_by_uuid(updated.id);
        assert!(found.is_ok());
        let found = found.unwrap();
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!("jhunt@example.com", found.email);
    }

    #[test]
    pub fn it_should_be_able_to_reuse_email_addresses_via_update() {
        let db = connect();

        let jhunt = db
            .create_collector(
                None,
                NewCollector {
                    username: "jhunt",
                    email: "james@example.com",
                },
                None,
            )
            .unwrap();
        db.find_collector_by_uuid(jhunt.id).unwrap().unwrap();

        let james = db
            .create_collector(
                None,
                NewCollector {
                    username: "james",
                    email: "other-james@example.com",
                },
                None,
            )
            .unwrap();
        db.find_collector_by_uuid(james.id).unwrap().unwrap();

        let updated = db.update_collector(
            &jhunt,
            CollectorUpdate {
                username: None,
                email: Some("other-james@example.com"),
                password: None,
            },
        );
        assert!(updated.is_ok());
        let updated = updated.unwrap();
        assert_eq!("other-james@example.com", updated.email);

        let found = db.find_collector_by_uuid(jhunt.id);
        assert!(found.is_ok());
        let found = found.unwrap();
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!("other-james@example.com", found.email);

        let found = db.find_collector_by_uuid(james.id);
        assert!(found.is_ok());
        let found = found.unwrap();
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!("other-james@example.com", found.email);
    }

    #[test]
    pub fn it_should_create_a_collection_for_a_new_collector() {
        let db = connect();

        let jhunt = db.create_collector(
            None,
            NewCollector {
                username: "jhunt",
                email: "james@example.com",
            },
            None,
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

        let jhunt = db
            .create_collector(
                None,
                NewCollector {
                    username: "jhunt",
                    email: "james@example.com",
                },
                None,
            )
            .unwrap();

        let txn = db.create_transaction(
            None,
            NewTransaction {
                collection: jhunt.id,
                dated: &NaiveDate::from_ymd(2020, 01, 14),
                gain: "1x XLN Opt\n",
                loss: "",
            },
        );
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

        let jhunt = db
            .create_collector(
                None,
                NewCollector {
                    username: "jhunt",
                    email: "james@example.com",
                },
                None,
            )
            .unwrap();

        let deck = db.create_deck(
            None,
            NewDeck {
                collector: jhunt.id,
                title: "Niv-Mizzet",
                description: "Draw a card, 1 damage to you...",
                main: "1x GRN Niv-Mizzet\n",
                side: "1x RNA Niv-Mizzet, Parun\n",
                maybe: "1x WAR Niv-Mizzet, Reborn\n",
            },
        );
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

    #[test]
    pub fn it_can_authenticate_a_collector() {
        let db = connect();

        db.create_collector(
            None,
            NewCollector {
                username: "jhunt",
                email: "james@example.com",
            },
            Some("sekrit"),
        )
        .expect("unable to create collector 'jhunt'");

        db.create_collector(
            None,
            NewCollector {
                username: "james",
                email: "other-james@example.com",
            },
            Some("my sekrit pas sword"),
        )
        .expect("unable to create collector 'james'");

        let who = db
            .authenticate_collector("jhunt", "sekrit")
            .expect("no error should occur while authenticating with correct credentials");
        assert!(who.is_some());
        let who = who.unwrap();
        assert_eq!(who.username, "jhunt");

        let who = db
            .authenticate_collector("james", "my sekrit pas sword")
            .expect("no error should occur while authenticating with correct credentials");
        assert!(who.is_some());
        let who = who.unwrap();
        assert_eq!(who.username, "james");

        let who = db
            .authenticate_collector("jhunt", "not their password")
            .expect("no error should occur while authenticating with incorrect credentials");
        assert!(who.is_none());

        let who = db
            .authenticate_collector("not jhunt", "sekrit")
            .expect("no error should occur while authenticating with an unknown username");
        assert!(who.is_none());

        let who = db
            .authenticate_collector("james", "sekrit")
            .expect("no error should occur while authenticating with incorrect credentials");
        assert!(who.is_none());
    }
}
