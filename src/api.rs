use chrono::{naive::NaiveDate, DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::db;

mod errors {
    error_chain! {}
}
pub use errors::Error;
use errors::*;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Object {
    NotFound(NotFound),
    Response(Response),
    Authenticated(Authenticated),
    Deck(Deck),
    Decks(Vec<Deck>),
    Transaction(Transaction),
}

impl Object {
    fn list_of_decks(other: Vec<db::Deck>) -> Self {
        let mut decks = vec![];
        for deck in other {
            decks.push(Deck::from(deck));
        }
        Object::Decks(decks)
    }
}

#[derive(Serialize)]
pub struct NotFound {
    kind: String,
    identity: String,
    message: String,
}

fn not_found(kind: &str, id: &str, msg: Option<&str>) -> Object {
    Object::NotFound(NotFound {
        kind: kind.to_string(),
        identity: id.to_string(),
        message: match msg {
            None => format!("no-such-{}", kind),
            Some(v) => v.to_string(),
        },
    })
}

#[derive(Serialize)]
pub struct Response {
    ok: bool,
    message: String,
}
#[derive(Serialize)]
pub struct Authenticated {
    pub session: String,
}

#[derive(Deserialize)]
pub struct AuthenticationAttempt {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SignupAttempt {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct TransactionCreationAttempt {
    pub dated: NaiveDate,
    pub gain: String,
    pub loss: String,
}

#[derive(Serialize)]
pub struct Transaction {
    pub id: String,
    pub collection: String,
    pub dated: NaiveDate,
    pub gain: String,
    pub loss: String,
}

impl std::convert::From<db::Transaction> for Transaction {
    fn from(other: db::Transaction) -> Self {
        Self {
            id: other.id.to_string(),
            collection: other.collection.to_string(),
            dated: other.dated,
            gain: other.gain,
            loss: other.loss,
        }
    }
}

#[derive(Deserialize)]
pub struct DeckCreationAttempt {
    pub title: String,
    pub description: String,
    pub main: String,
    pub side: String,
    pub maybe: String,
}

#[derive(Deserialize)]
pub struct DeckUpdateAttempt {
    pub title: Option<String>,
    pub description: Option<String>,
    pub main: Option<String>,
    pub side: Option<String>,
    pub maybe: Option<String>,
}

#[derive(Serialize)]
pub struct Deck {
    pub id: String,        // uuid
    pub collector: String, // uuid
    pub title: String,
    pub description: String,
    pub main: String,
    pub side: String,
    pub maybe: String,
    pub lineage: String, // uuid
    pub ordinal: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl std::convert::From<db::Deck> for Deck {
    fn from(other: db::Deck) -> Deck {
        Deck {
            id: other.id.to_string(),
            collector: other.collector.to_string(),
            title: other.title,
            description: other.description,
            main: other.main,
            side: other.side,
            maybe: other.maybe,
            lineage: other.lineage.to_string(),
            ordinal: other.ordinal,
            created_at: other.created_at,
            updated_at: other.updated_at,
        }
    }
}

pub struct API {
    db: db::Database,
}

impl API {
    pub fn new(dat: db::Database) -> API {
        API { db: dat }
    }

    pub fn authenticate(&self, a: AuthenticationAttempt) -> Result<Object> {
        match self.db.authenticate_collector(&a.username, &a.password) {
            Ok(Some(who)) => {
                let mut session = db::Session::new(None);
                session.set("user-id", &who.id.to_string());
                match self.db.set_session(session) {
                    Ok(session) => Ok(Object::Authenticated(Authenticated {
                        session: session.id.to_string(),
                    })),
                    _ => Ok(Object::Response(Response {
                        ok: false,
                        message: "session-failed".to_string(),
                    })),
                }
            }
            _ => Ok(Object::Response(Response {
                ok: false,
                message: "authentication-failed".to_string(),
            })),
        }
    }

    pub fn signup(&self, a: SignupAttempt) -> Result<Object> {
        let who = self
            .db
            .create_collector(
                None,
                db::NewCollector {
                    username: &a.username,
                    email: &a.email,
                },
                Some(&a.password),
            )
            .chain_err(|| "unable to sign up new collector")?;

        let mut session = db::Session::new(None);
        session.set("fresh", "signup");
        session.set("user-id", &who.id.to_string());
        match self.db.set_session(session) {
            Ok(session) => Ok(Object::Authenticated(Authenticated {
                session: session.id.to_string(),
            })),
            _ => Ok(Object::Response(Response {
                ok: false,
                message: "signup-failed".to_string(),
            })),
        }
    }

    pub fn post_transaction(&self, cid: &str, new: TransactionCreationAttempt) -> Result<Object> {
        let collection = match self
            .db
            .find_collection_by_uuid(
                Uuid::parse_str(cid).chain_err(|| "unable to parse collection uuid")?,
            )
            .chain_err(|| "unable to find collection to post transaction to")?
        {
            Some(collection) => collection,
            None => return Ok(not_found("collection", cid, None)),
        };

        match self.db.create_transaction(
            None,
            db::NewTransaction {
                collection: collection.id,
                dated: &new.dated,
                gain: &new.gain,
                loss: &new.loss,
            },
        ) {
            Ok(txn) => Ok(Object::Transaction(Transaction::from(txn))),
            _ => Ok(Object::Response(Response {
                ok: false,
                message: "transaction-creation-failed".to_string(),
            })),
        }
    }

    pub fn create_deck(&self, uid: &str, new: DeckCreationAttempt) -> Result<Object> {
        let collector = match self
            .db
            .find_collector_by_uuid(
                Uuid::parse_str(uid).chain_err(|| "unable to parse collector uuid")?,
            )
            .chain_err(|| "unable to find collector to create deck for")?
        {
            Some(collector) => collector,
            None => return Ok(not_found("collector", uid, None)),
        };

        match self.db.create_deck(
            None,
            db::NewDeck {
                collector: collector.id,
                title: &new.title,
                description: &new.description,
                main: &new.main,
                side: &new.side,
                maybe: &new.maybe,
            },
        ) {
            Ok(deck) => Ok(Object::Deck(Deck::from(deck))),
            _ => Ok(Object::Response(Response {
                ok: false,
                message: "deck-creation-failed".to_string(),
            })),
        }
    }

    pub fn update_deck(&self, uid: &str, did: &str, upd: DeckUpdateAttempt) -> Result<Object> {
        let collector = match self
            .db
            .find_collector_by_uuid(
                Uuid::parse_str(uid).chain_err(|| "unable to parse collector uuid")?,
            )
            .chain_err(|| "unable to find collector to update deck for")?
        {
            Some(collector) => collector,
            None => return Ok(not_found("collector", uid, None)),
        };

        let deck = match self.db.find_deck_by_uuid(
                collector.id,
                Uuid::parse_str(did).chain_err(|| "unable to parse deck uuid")?,
            ).chain_err(|| "unable to find deck to update")?
        {
            Some(deck) => deck,
            None => return Ok(not_found("deck", did, None)),
        };

        match self.db.update_deck(
            deck.id,
            db::UpdateDeck {
                title: upd.title,
                description: upd.description,
                main: upd.main,
                side: upd.side,
                maybe: upd.maybe,
            },
        ) {
            Ok(Some(deck)) => Ok(Object::Deck(Deck::from(deck))),
            Ok(None) => Ok(not_found("deck", did, None)),
            _ => Ok(Object::Response(Response {
                ok: false,
                message: "deck-update-failed".to_string(),
            })),
        }
    }

    pub fn delete_deck(&self, uid: &str, did: &str) -> Result<Object> {
        let collector = match self
            .db
            .find_collector_by_uuid(
                Uuid::parse_str(uid).chain_err(|| "unable to parse collector uuid")?,
            )
            .chain_err(|| "unable to find collector to update deck for")?
        {
            Some(collector) => collector,
            None => return Ok(not_found("collector", uid, None)),
        };

        let deck = match self.db.find_deck_by_uuid(
                collector.id,
                Uuid::parse_str(did).chain_err(|| "unable to parse deck uuid")?,
            ).chain_err(|| "unable to find deck to update")?
        {
            Some(deck) => deck,
            None => return Ok(not_found("deck", did, None)),
        };

        match self.db.delete_deck(deck.id) {
            Ok(_) => Ok(Object::Response(Response {
                ok: true,
                message: "deck-removed".to_string(),
            })),
            _ => Ok(Object::Response(Response {
                ok: false,
                message: "deck-removal-failed".to_string(),
            })),
        }
    }

    pub fn retrieve_decks_for_collector(&self, uid: &str) -> Result<Object> {
        let collector = match self
            .db
            .find_collector_by_uuid(
                Uuid::parse_str(uid).chain_err(|| "unable to parse collector uuid")?,
            )
            .chain_err(|| "unable to find collector to retrieve decks from")?
        {
            Some(collector) => collector,
            None => return Ok(not_found("collector", uid, None)),
        };

        Ok(Object::list_of_decks(self.db
            .find_decks_for_collector(collector.id)
            .chain_err(|| "unable to find decks by collector uuid")?))
    }

    pub fn retrieve_deck(&self, uid: &str, did: &str) -> Result<Object> {
        let collector = match self
            .db
            .find_collector_by_uuid(
                Uuid::parse_str(uid).chain_err(|| "unable to parse collector uuid")?,
            )
            .chain_err(|| "unable to find collector to retrieve deck for")?
        {
            Some(collector) => collector,
            None => return Ok(not_found("collector", uid, None)),
        };

        match self
            .db
            .find_deck_by_uuid(
                collector.id,
                Uuid::parse_str(did).chain_err(|| "unable to parse deck uuid")?,
            )
            .chain_err(|| "unable to find deck by uuid")?
        {
            Some(deck) => Ok(Object::Deck(Deck::from(deck))),
            None => Ok(not_found("deck", did, None)),
        }
    }
}
