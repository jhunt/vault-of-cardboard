use super::schema::{collections, collectors, decks, transactions};
use chrono::{naive::NaiveDate, DateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

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
    pub lineage: Uuid,
    pub ordinal: &'a i32,
}

pub struct Database {
    conn: PgConnection,
}

embed_migrations!("migrations/");

fn gen_uuid(id: Option<Uuid>) -> Uuid {
    match id {
        Some(id) => id,
        None => Uuid::new_v4(),
    }
}

impl Database {
    pub fn connect(url: &str) -> Database {
        let db = Database {
            conn: PgConnection::establish(url).unwrap(),
        };

        embedded_migrations::run(&db.conn).unwrap();
        db
    }

    pub fn create_collector(&self, id: Option<Uuid>, new: NewCollector) -> Collector {
        let id = gen_uuid(id);
        let collector = diesel::insert_into(collectors::table)
            .values((&new, collectors::dsl::id.eq(id)))
            .get_result(&self.conn)
            .unwrap();

        diesel::insert_into(collections::table)
            .values(&NewCollection { id, collector: id })
            .get_result::<Collection>(&self.conn)
            .unwrap();

        collector
    }

    pub fn update_collector(&self, obj: &Collector, upd: CollectorUpdate) -> Collector {
        diesel::update(obj).set(&upd).get_result(&self.conn).unwrap()
    }

    pub fn create_transaction(&self, id: Option<Uuid>, new: NewTransaction) -> Transaction {
        diesel::insert_into(transactions::table)
            .values((&new, transactions::dsl::id.eq(gen_uuid(id))))
            .get_result(&self.conn)
            .unwrap()
    }

    //pub fn update_transaction
    //pub fn delete_transaction

    pub fn create_deck(&self, id: Option<Uuid>, new: NewDeck) -> Deck {
        let now = Utc::now();
        diesel::insert_into(decks::table)
            .values((
                &new,
                decks::dsl::id.eq(gen_uuid(id)),
                decks::dsl::created_at.eq(now),
                decks::dsl::updated_at.eq(now),
            ))
            .get_result(&self.conn)
            .unwrap()
    }

    //pub fn snapshot_deck
    //pub fn update_deck
    //pub fn delete_deck
}
