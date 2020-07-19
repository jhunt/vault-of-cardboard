use super::schema::{collections, collectors, decks, transactions};
use bcrypt;
use chrono::{naive::NaiveDate, DateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use redis;
use serde::Serialize;
use serde_json::json;
use std::fs::{self, File};
use std::io::{self, Read, Seek, Write};
use std::path::{Path, PathBuf};
use uuid::Uuid;

use crate::prelude::*;
use crate::{card, cdif};

mod errors {
    error_chain! {}
}
pub use errors::Error;
use errors::*;

const PWHASH_COST: u32 = 10;

pub struct FStore {
    root: PathBuf,
}

impl FStore {
    pub fn new(root: &Path) -> Self {
        Self {
            root: PathBuf::from(root),
        }
    }

    fn path_to(&self, filename: &str) -> PathBuf {
        let mut pb = self.root.clone();
        pb.push(filename);
        pb
    }

    pub fn create(&self, filename: &str, contents: &str) -> Result<()> {
        let pb = self.path_to(filename);

        fs::create_dir_all(
            pb.parent()
                .chain_err(|| "failed to construct path for file storage")?,
        )
        .chain_err(|| "failed to create parent directories for file storage")?;

        let mut file = File::create(
            pb.to_str()
                .chain_err(|| "failed to construct a path for file storage")?,
        )
        .chain_err(|| "failed to create file")?;
        write!(file, "{}", contents).chain_err(|| "failed to write to file")?;

        Ok(())
    }

    pub fn get_as_reader(&self, filename: &str) -> Result<File> {
        let pb = self.path_to(filename);
        Ok(File::open(
            pb.to_str()
                .chain_err(|| "failed to construct a path for file retrieval")?,
        )
        .chain_err(|| "failed to open file")?)
    }

    pub fn get_as_string(&self, filename: &str) -> Result<String> {
        let mut file = self.get_as_reader(filename)?;

        let mut s = String::new();
        file.read_to_string(&mut s)
            .chain_err(|| "failed to read from file")?;
        Ok(s)
    }

    pub fn append_to_json_list<T: Serialize>(&self, filename: &str, item: T) -> Result<()> {
        let pb = self.path_to(filename);
        let mut file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(
                pb.to_str()
                    .chain_err(|| "failed to construct a path for file modifucation")?,
            )
            .chain_err(|| "failed to open file")?;

        file.seek(io::SeekFrom::End(-2))
            .chain_err(|| "failed to seek to end of json list file")?;

        write!(file, ",{}]]", json!(item)).chain_err(|| "failed to append to json list file")?;

        Ok(())
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

#[derive(Identifiable, Queryable)]
pub struct Transaction {
    pub id: Uuid,
    pub collection: Uuid,
    pub dated: NaiveDate,
    pub summary: String,
    pub notes: String,
    pub disposition: String,
    pub gain: String,
    pub loss: String,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub paid: Option<i32>,
}

impl Transaction {
    fn metadata_value_as_u32(&self, key: &str, or: u32) -> u32 {
        match self.metadata.get(key) {
            Some(serde_json::Value::Number(number)) => match number.as_f64() {
                Some(n) => n as u32,
                _ => or,
            },
            _ => or,
        }
    }

    pub fn total_card_gain(&self) -> u32 {
        self.metadata_value_as_u32("total_gain", 0)
    }

    pub fn total_card_loss(&self) -> u32 {
        self.metadata_value_as_u32("total_loss", 0)
    }

    pub fn unique_card_gain(&self) -> u32 {
        self.metadata_value_as_u32("unique_gain", 0)
    }

    pub fn unique_card_loss(&self) -> u32 {
        self.metadata_value_as_u32("unique_loss", 0)
    }

    pub fn set_gain(&self) -> Vec<String> {
        match self.metadata.get("set_gain") {
            Some(serde_json::Value::Array(v)) => {
                v.iter().map(|x| x.as_str().unwrap().to_string()).collect()
            }
            _ => vec![],
        }
    }
    pub fn set_loss(&self) -> Vec<String> {
        match self.metadata.get("set_loss") {
            Some(serde_json::Value::Array(v)) => {
                v.iter().map(|x| x.as_str().unwrap().to_string()).collect()
            }
            _ => vec![],
        }
    }
}

#[derive(Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction<'a> {
    pub collection: Uuid,
    pub summary: &'a str,
    pub notes: &'a str,
    pub dated: &'a NaiveDate,
    pub disposition: &'a str,
    pub gain: &'a str,
    pub loss: &'a str,
    pub paid: Option<i32>,
}

#[derive(AsChangeset)]
#[table_name = "transactions"]
pub struct UpdateTransaction {
    pub summary: Option<String>,
    pub notes: Option<String>,
    pub dated: Option<NaiveDate>,
    pub disposition: Option<String>,
    pub gain: Option<String>,
    pub loss: Option<String>,
    pub paid: Option<Option<i32>>,
}

#[derive(Identifiable, Queryable)]
pub struct Deck {
    pub id: Uuid,
    pub collector: Uuid,
    pub title: String,
    pub code: String,
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
    pub code: &'a str,
    pub description: &'a str,
    pub main: &'a str,
    pub side: &'a str,
    pub maybe: &'a str,
}

#[derive(AsChangeset)]
#[table_name = "decks"]
pub struct UpdateDeck {
    pub title: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub main: Option<String>,
    pub side: Option<String>,
    pub maybe: Option<String>,
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
    fs: FStore,
    idle: u32, // in seconds
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
    pub fn migrate(pg: &str) -> Result<()> {
        let pg = PgConnection::establish(pg).chain_err(|| "unable to connect to database")?;
        embedded_migrations::run(&pg).chain_err(|| "failed to run database migrations")?;
        Ok(())
    }

    // Connect to a DSN (must be PostgreSQL) and run migrations.
    pub fn connect(pg: &str, rd: &str, fsroot: &Path, idle: u32) -> Result<Database> {
        Ok(Database {
            pg: PgConnection::establish(pg).chain_err(|| "unable to connect to database")?,
            rd: redis::Client::open(rd).chain_err(|| "unable to connect to session store")?,
            fs: FStore::new(fsroot),
            idle: idle,
        })
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

    pub fn get_file(&self, rel: &str) -> Result<std::fs::File> {
        Ok(self
            .fs
            .get_as_reader(rel)
            .chain_err(|| "unable to retrieve file from filesystem")?)
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
        let cmd = cmd.arg(&key).arg(self.idle.to_string()); // FIXME
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

        self.fs
            .create(
                &format!("c/{}/_/collection.json", id.to_string()),
                r#"[[],[[]]]"#,
            )
            .chain_err(|| "failed to create initial collection cache file")?;

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
        match collectors::dsl::collectors
            .find(id)
            .get_result::<Collector>(&self.pg)
        {
            Ok(collector) => Ok(Some(collector)),
            Err(diesel::NotFound) => Ok(None),
            Err(e) => Err(Error::with_chain(
                e,
                "failed to retrieve collector from database",
            )),
        }
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
        match collections::dsl::collections
            .find(id)
            .get_result::<Collection>(&self.pg)
        {
            Ok(collection) => Ok(Some(collection)),
            Err(diesel::NotFound) => Ok(None),
            Err(e) => Err(Error::with_chain(
                e,
                "failed to retrieve collection from database",
            )),
        }
    }

    pub fn find_transactions_for_collection(&self, cid: Uuid) -> Result<Vec<Transaction>> {
        Ok(transactions::dsl::transactions
            .filter(transactions::dsl::collection.eq(cid))
            .get_results::<Transaction>(&self.pg)
            .chain_err(|| "unable to retrieve transactions for collection uuid")?)
    }

    // Find a Transaction by its UUID.
    pub fn find_transaction_by_uuid(&self, cid: Uuid, id: Uuid) -> Result<Option<Transaction>> {
        match transactions::dsl::transactions
            .find(id)
            .filter(transactions::dsl::collection.eq(cid))
            .get_result::<Transaction>(&self.pg)
        {
            Ok(transaction) => Ok(Some(transaction)),
            Err(diesel::NotFound) => Ok(None),
            Err(e) => Err(Error::with_chain(
                e,
                "failed to retrieve transaction from database",
            )),
        }
    }

    // Create a new Transaction.
    pub fn create_transaction(&self, id: Option<Uuid>, new: NewTransaction) -> Result<Transaction> {
        let gain = cdif::File::from_string(&new.gain)
            .chain_err(|| "unable to parse gained cards from transaction")?;
        let loss = cdif::File::from_string(&new.loss)
            .chain_err(|| "unable to parse lost cards from transaction")?;

        let mut meta = serde_json::Map::new();

        let (total, unique) = gain.count();
        meta.insert("total_gain".to_string(), json!(total as f64));
        meta.insert("unique_gain".to_string(), json!(unique as f64));
        meta.insert("set_gain".to_string(), json!(gain.unique_sets()));

        let (total, unique) = loss.count();
        meta.insert("total_loss".to_string(), json!(total as f64));
        meta.insert("unique_loss".to_string(), json!(unique as f64));
        meta.insert("set_loss".to_string(), json!(loss.unique_sets()));

        let txn = diesel::insert_into(transactions::table)
            .values((
                &new,
                transactions::dsl::metadata.eq(serde_json::Value::Object(meta)),
                transactions::dsl::id.eq(gen_uuid(id)),
            ))
            .get_result(&self.pg)
            .chain_err(|| "failed to insert transaction record into database")?;

        // update the collection with new, resolved CDIF
        self.apply_collection_credit(new.collection, gain)
            .chain_err(|| "unable to apply new gains from transaction")?;

        // update the collection again for any losses
        self.apply_collection_debit(new.collection, loss)
            .chain_err(|| "unable to apply new losses from transaction")?;

        Ok(txn)
    }

    pub fn update_transaction(
        &self,
        obj: &Transaction,
        upd: UpdateTransaction,
    ) -> Result<Transaction> {
        let mut meta = match obj.metadata.as_object() {
            Some(meta) => meta.clone(),
            _ => serde_json::Map::new(),
        };

        let gain = match &upd.gain {
            None => None,
            Some(s) => {
                let cdif = cdif::File::from_string(&s)
                    .chain_err(|| "failed to parse updated gains during transaction update")?;

                let (total, unique) = cdif.count();
                meta.insert("total_gain".to_string(), json!(total as f64));
                meta.insert("unique_gain".to_string(), json!(unique as f64));
                meta.insert("set_gain".to_string(), json!(cdif.unique_sets()));

                Some(cdif)
            }
        };

        let loss = match &upd.loss {
            None => None,
            Some(s) => {
                let cdif = cdif::File::from_string(&s)
                    .chain_err(|| "failed to parse updated losses during transaction update")?;

                let (total, unique) = cdif.count();
                meta.insert("total_loss".to_string(), json!(total as f64));
                meta.insert("unique_loss".to_string(), json!(unique as f64));
                meta.insert("set_loss".to_string(), json!(cdif.unique_sets()));

                Some(cdif)
            }
        };

        let txn = diesel::update(obj)
            .set((
                &upd,
                transactions::dsl::metadata.eq(serde_json::Value::Object(meta)),
                transactions::dsl::updated_at.eq(Utc::now()),
            ))
            .get_result(&self.pg)
            .chain_err(|| "failed to update transaction record in database")?;

        match gain {
            None => (),
            Some(now) => {
                let then = cdif::File::from_string(&obj.gain)
                    .chain_err(|| "failed to parse previous gains during transaction update")?;

                self.apply_collection_credit(obj.collection, cdif::File::diff(&then, &now))?;
            }
        };

        match loss {
            None => (),
            Some(now) => {
                let then = cdif::File::from_string(&obj.loss)
                    .chain_err(|| "failed to parse previous losses during transaction update")?;

                self.apply_collection_debit(obj.collection, cdif::File::diff(&then, &now))?;
            }
        };

        Ok(txn)
    }

    pub fn delete_transaction(&self, id: Uuid) -> Result<()> {
        diesel::delete(transactions::dsl::transactions.filter(transactions::dsl::id.eq(id)))
            .execute(&self.pg)
            .chain_err(|| "failed to delete transaction record from database")?;
        Ok(())
    }

    fn apply_collection_diff(&self, id: Uuid, cards: cdif::File, credit: bool) -> Result<()> {
        let mut f = self
            .fs
            .get_as_reader("lookup.json")
            .chain_err(|| "failed to retrieve card name -> print id lookup table")?;

        let lookup = card::Map::from_reader(&mut f)
            .chain_err(|| "failed to retrieve card name -> print id lookup table")?;

        let mut delta = card::Pile::resolve(cards, lookup);
        if !credit {
            delta.invert();
        }
        self.fs
            .append_to_json_list(
                &format!("c/{}/_/collection.json", id.to_string()),
                delta.cards,
            )
            .chain_err(|| "failed to append json object to collection json file")?;

        Ok(())
    }

    pub fn apply_collection_credit(&self, id: Uuid, credit: cdif::File) -> Result<()> {
        self.apply_collection_diff(id, credit, true)
    }

    pub fn apply_collection_debit(&self, id: Uuid, debit: cdif::File) -> Result<()> {
        self.apply_collection_diff(id, debit, false)
    }

    pub fn find_decks_for_collector(&self, uid: Uuid) -> Result<Vec<Deck>> {
        Ok(decks::dsl::decks
            .filter(decks::dsl::collector.eq(uid))
            .get_results::<Deck>(&self.pg)
            .chain_err(|| "unable to retrieve decks for collector uuid")?)
    }

    pub fn find_deck_by_uuid(&self, uid: Uuid, id: Uuid) -> Result<Option<Deck>> {
        match decks::dsl::decks
            .find(id)
            .filter(decks::dsl::collector.eq(uid))
            .get_result::<Deck>(&self.pg)
        {
            Ok(deck) => Ok(Some(deck)),
            Err(diesel::NotFound) => Ok(None),
            Err(e) => Err(Error::with_chain(
                e,
                "failed to retrieve deck record from database",
            )),
        }
    }

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

    pub fn update_deck(&self, obj: &Deck, upd: UpdateDeck) -> Result<Deck> {
        Ok(diesel::update(obj)
            .set((&upd, decks::dsl::updated_at.eq(Utc::now())))
            .get_result(&self.pg)
            .chain_err(|| "failed to update deck record in database")?)
    }

    pub fn delete_deck(&self, id: Uuid) -> Result<()> {
        diesel::delete(decks::dsl::decks.filter(decks::dsl::id.eq(id)))
            .execute(&self.pg)
            .chain_err(|| "failed to delete deck record from database")?;
        Ok(())
    }

    //pub fn snapshot_deck
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;
    use tempdir::TempDir;
    use uuid::Uuid;

    fn connect() -> (TempDir, Database) {
        use std::env;

        let pg =
            env::var("VCB_DATABASE_URL").expect("VCB_DATABASE_URL must be set in the environment.");

        let rd = env::var("VCB_REDIS_URL").expect("VCB_REDIS_URL must be set in the environment.");

        let fs = TempDir::new("vcb-test").expect("Failed to create temp directory");

        let db = Database::connect(&pg, &rd, &fs.path(), 3600).unwrap();
        db.testmode().unwrap();
        db.fs
            .create(
                "lookup.json",
                r#"
{
  "XLN * Opt": "xln-opt-fake-id",
  "GRN * Radical Idea": "grn-rad-fake-id"
}
"#,
            )
            .unwrap();
        (fs, db)
    }

    #[test]
    fn should_be_able_to_create_a_session() {
        let (_tmp, db) = connect();

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
    fn should_be_able_to_create_a_collector() {
        let (_tmp, db) = connect();

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
    fn should_not_be_able_to_reuse_usernames() {
        let (_tmp, db) = connect();

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
    fn should_be_able_to_reuse_email_addresses() {
        let (_tmp, db) = connect();

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
    #[ignore]
    fn should_be_able_to_reuse_passwords() {
        let (_tmp, db) = connect();

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
    fn should_be_able_to_update_a_collectors_username() {
        let (_tmp, db) = connect();

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
    fn should_not_be_able_to_reuse_usernames_via_update() {
        let (_tmp, db) = connect();

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
    fn should_be_able_to_update_a_collectors_email() {
        let (_tmp, db) = connect();

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
    fn should_be_able_to_reuse_email_addresses_via_update() {
        let (_tmp, db) = connect();

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
    fn should_create_a_collection_for_a_new_collector() {
        let (_tmp, db) = connect();

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

    fn vec_to_set(src: Vec<String>) -> HashSet<String> {
        let mut dst = HashSet::new();
        for x in src {
            dst.insert(x);
        }
        dst
    }

    #[test]
    pub fn can_create_a_transaction() {
        let (_tmp, db) = connect();

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
                summary: "opting for ixalan",
                disposition: "buy",
                notes: "this oughta be good",
                collection: jhunt.id,
                dated: &NaiveDate::from_ymd(2020, 01, 14),
                gain: "1x XLN Opt\n",
                loss: "",
                paid: None,
            },
        );
        assert!(txn.is_ok());
        let txn = txn.unwrap();

        assert_eq!(txn.collection, jhunt.id);
        assert_eq!(txn.summary, "opting for ixalan");
        assert_eq!(txn.notes, "this oughta be good");
        assert_eq!(txn.disposition, "buy");
        assert_eq!(txn.dated, NaiveDate::from_ymd(2020, 01, 14));
        assert_eq!(txn.gain, "1x XLN Opt\n");
        assert_eq!(txn.loss, "");
        assert_eq!(txn.total_card_gain(), 1);
        assert_eq!(txn.unique_card_gain(), 1);
        assert_eq!(txn.total_card_loss(), 0);
        assert_eq!(txn.unique_card_loss(), 0);

        assert_eq!(txn.set_loss().len(), 0);
        assert_eq!(txn.set_gain().len(), 1);
        let sets = vec_to_set(txn.set_gain());
        assert!(sets.contains("XLN"));

        let updated = db
            .update_transaction(
                &txn,
                UpdateTransaction {
                    summary: None,
                    notes: None,
                    dated: None,
                    disposition: None,
                    gain: Some("1x XLN Opt\n3x GRN Radical Idea\n".to_string()),
                    loss: None,
                    paid: None,
                },
            )
            .unwrap();
        assert_eq!(updated.collection, jhunt.id);
        assert_eq!(updated.dated, NaiveDate::from_ymd(2020, 01, 14));
        assert_eq!(updated.summary, "opting for ixalan");
        assert_eq!(updated.notes, "this oughta be good");
        assert_eq!(updated.disposition, "buy");
        assert_eq!(updated.gain, "1x XLN Opt\n3x GRN Radical Idea\n");
        assert_eq!(updated.loss, "");
        assert_eq!(updated.total_card_gain(), 4);
        assert_eq!(updated.unique_card_gain(), 2);
        assert_eq!(updated.total_card_loss(), 0);
        assert_eq!(updated.unique_card_loss(), 0);

        assert_eq!(updated.set_loss().len(), 0);
        assert_eq!(updated.set_gain().len(), 2);
        let sets = vec_to_set(updated.set_gain());
        assert!(sets.contains("XLN"));
        assert!(sets.contains("GRN"));
    }

    #[test]
    pub fn can_create_a_deck() {
        let (_tmp, db) = connect();

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
                code: "niv",
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
    #[ignore]
    pub fn can_authenticate_a_collector() {
        let (_tmp, db) = connect();

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

    #[test]
    fn should_be_able_to_create_files() {
        let fs = TempDir::new("data-fs-test").expect("failed to create tempdir");
        let store = FStore::new(fs.path());

        let s = store.get_as_string("test");
        assert!(s.is_err());

        assert!(store.create("test", r#"[[""]]"#).is_ok());
        let s = store.get_as_string("test");
        assert!(s.is_ok());
        let s = s.unwrap();
        assert_eq!(s, r#"[[""]]"#);
    }

    #[test]
    fn should_create_intervening_parent_directories() {
        let fs = TempDir::new("data-fs-test").expect("failed to create tempdir");
        let store = FStore::new(fs.path());

        let key = "a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z.json";
        assert!(store.create(key, "FOO").is_ok());
        let s = store.get_as_string(key);
        assert!(s.is_ok());
        let s = s.unwrap();
        assert_eq!(s, "FOO");
    }

    #[test]
    fn should_be_able_to_append_new_json() {
        let fs = TempDir::new("data-fs-test").expect("failed to create tempdir");
        let store = FStore::new(fs.path());

        assert!(store
            .create("something.json", r#"[["initial"],[[]]]"#)
            .is_ok());
        assert!(store
            .append_to_json_list("something.json", vec!["new", "things"])
            .is_ok());
        let s = store.get_as_string("something.json");
        assert!(s.is_ok());
        let s = s.unwrap();
        assert_eq!(s, r#"[["initial"],[[],["new","things"]]]"#);
    }
}
