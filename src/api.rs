use chrono::{naive::NaiveDate, DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

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
    Goal(Goal),
    Goals(Vec<Goal>),
    Deck(Deck),
    Decks(Vec<Deck>),
    Transaction(Transaction),
    Transactions(Vec<Transaction>),
}

impl Object {
    fn ok(msg: &str) -> Self {
        Self::Response(Response {
            ok: true,
            message: msg.to_string(),
        })
    }

    fn fail(msg: &str) -> Self {
        Self::Response(Response {
            ok: false,
            message: msg.to_string(),
        })
    }

    fn list_of_goals(other: Vec<db::Goal>) -> Self {
        let mut goals = vec![];
        for goal in other {
            goals.push(Goal::from(goal));
        }
        Self::Goals(goals)
    }

    fn list_of_decks(other: Vec<db::Deck>) -> Self {
        let mut decks = vec![];
        for deck in other {
            decks.push(Deck::from(deck));
        }
        Self::Decks(decks)
    }

    fn list_of_transactions(other: Vec<db::Transaction>) -> Self {
        let mut transactions = vec![];
        for txn in other {
            transactions.push(Transaction::from(txn));
        }
        Self::Transactions(transactions)
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
    pub uid: String,
    pub username: String,
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

impl SignupAttempt {
    pub fn valid(&self) -> Result<()> {
        let re = Regex::new(r"^\S+@\S+\.\S+$").unwrap();
        if self.username.graphemes(true).count() < 4 {
            Err("username-too-short".into())
        } else if !re.is_match(&self.email) {
            Err("invalid-email".into())
        } else if self.password.graphemes(true).count() < 8 {
            Err("password-too-short".into())
        } else {
            Ok(())
        }
    }
}

#[derive(Deserialize)]
pub struct TransactionCreationAttempt {
    pub summary: String,
    pub notes: String,
    pub dated: NaiveDate,
    pub disposition: String,
    pub paid: Option<i32>,
    pub gain: String,
    pub loss: String,
}

#[derive(Deserialize)]
pub struct TransactionUpdateAttempt {
    pub summary: Option<String>,
    pub notes: Option<String>,
    pub dated: Option<NaiveDate>,
    pub disposition: Option<String>,
    pub gain: Option<String>,
    pub loss: Option<String>,
    pub paid: Option<i32>,
}

#[derive(Serialize)]
pub struct Transaction {
    pub id: String,
    pub collection: String,
    pub summary: String,
    pub notes: String,
    pub dated: NaiveDate,
    pub disposition: String,
    pub gain: String,
    pub loss: String,
    pub paid: Option<i32>,

    pub total_card_gain: u32,
    pub total_card_loss: u32,
    pub unique_card_gain: u32,
    pub unique_card_loss: u32,
    pub set_loss: Vec<String>,
    pub set_gain: Vec<String>,
}

impl std::convert::From<db::Transaction> for Transaction {
    fn from(other: db::Transaction) -> Self {
        Self {
            id: other.id.to_string(),
            collection: other.collection.to_string(),
            dated: other.dated,
            summary: other.summary.to_string(),
            notes: other.notes.to_string(),

            total_card_gain: other.total_card_gain(),
            total_card_loss: other.total_card_loss(),
            unique_card_gain: other.unique_card_gain(),
            unique_card_loss: other.unique_card_loss(),
            set_loss: other.set_loss(),
            set_gain: other.set_gain(),

            disposition: other.disposition.to_string(),
            gain: other.gain,
            loss: other.loss,
            paid: other.paid,
        }
    }
}

#[derive(Deserialize)]
pub struct GoalCreationAttempt {
    pub name: String,
    pub ordinal: i32,
    pub target: String,
    pub goal: String,
    pub total: Option<i32>,
    pub progress: Option<i32>,
}

#[derive(Deserialize)]
pub struct GoalUpdateAttempt {
    pub name: Option<String>,
    pub ordinal: Option<i32>,
    pub target: Option<String>,
    pub goal: Option<String>,
    pub total: Option<Option<i32>>,
    pub progress: Option<Option<i32>>,
}

#[derive(Serialize)]
pub struct Goal {
    pub id: String,        // uuid
    pub collector: String, // uuid
    pub name: String,
    pub ordinal: i32,
    pub target: String,
    pub goal: String,
    pub total: Option<i32>,
    pub progress: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl std::convert::From<db::Goal> for Goal {
    fn from(other: db::Goal) -> Goal {
        Goal {
            id: other.id.to_string(),
            collector: other.collector.to_string(),
            name: other.name,
            ordinal: other.ordinal,
            target: other.target,
            goal: other.goal,
            total: other.total,
            progress: other.progress,
            created_at: other.created_at,
            updated_at: other.updated_at,
        }
    }
}

#[derive(Deserialize)]
pub struct DeckCreationAttempt {
    pub title: String,
    pub code: String,
    pub description: String,
    pub main: String,
    pub side: String,
    pub maybe: String,
}

#[derive(Deserialize)]
pub struct DeckUpdateAttempt {
    pub title: Option<String>,
    pub code: Option<String>,
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
    pub code: String,
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
            code: other.code,
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

    pub fn guard(&self, sid: Option<String>, uid: &str) -> Option<i16> {
        let sid = match sid {
            None => return Some(401),
            Some(sid) => match Uuid::parse_str(&sid) {
                Err(_) => return Some(401),
                Ok(sid) => sid,
            },
        };

        match self.db.get_session(sid) {
            Err(_) => Some(403),
            Ok(None) => Some(401),
            Ok(Some(session)) => match session.attrs.get("user-id") {
                Some(id) if id == uid => None,
                Some(_) => Some(403),
                None => Some(401),
            },
        }
    }

    pub fn file(&self, rel: &str) -> Result<std::fs::File> {
        Ok(self.db.get_file(rel).chain_err(|| "unable to get file")?)
    }

    pub fn whoami(&self, sid: Option<String>) -> Object {
        let sid = match sid {
            None => return Object::fail("not-authenticated"),
            Some(sid) => match Uuid::parse_str(&sid) {
                Ok(sid) => sid,
                _ => return Object::fail("invalid-session-id"),
            },
        };

        let session = match self.db.get_session(sid) {
            Ok(Some(session)) => session,
            _ => return Object::fail("invalid-session-id"),
        };

        let uid = match session.attrs.get("user-id") {
            Some(uid) => match Uuid::parse_str(&uid) {
                Ok(uid) => uid,
                _ => return Object::fail("invalid-session-id"),
            },
            _ => return Object::fail("invalid-session-id"),
        };

        let who = match self.db.find_collector_by_uuid(uid) {
            Ok(Some(collector)) => collector,
            _ => return Object::fail("invalid-session-id"),
        };

        Object::Authenticated(Authenticated {
            uid: who.id.to_string(),
            username: who.username.to_string(),
            session: session.id.to_string(),
        })
    }

    pub fn authenticate(&self, a: AuthenticationAttempt) -> Result<Object> {
        match self.db.authenticate_collector(&a.username, &a.password) {
            Ok(Some(who)) => {
                let mut session = db::Session::new(None);
                session.set("user-id", &who.id.to_string());
                match self.db.set_session(session) {
                    Ok(session) => Ok(Object::Authenticated(Authenticated {
                        uid: who.id.to_string(),
                        username: who.username.to_string(),
                        session: session.id.to_string(),
                    })),
                    _ => Ok(Object::fail("session-failed")),
                }
            }
            _ => Ok(Object::fail("authentication-failed")),
        }
    }

    pub fn signup(&self, a: SignupAttempt) -> Result<Object> {
        match a.valid() {
            Ok(_) => {
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
                        uid: who.id.to_string(),
                        username: who.username.to_string(),
                        session: session.id.to_string(),
                    })),
                    _ => Ok(Object::fail("signup-failed")),
                }
            }
            Err(e) => Ok(Object::fail(&e.to_string())),
        }
    }

    pub fn retrieve_transactions_for_collection(&self, cid: &str) -> Result<Object> {
        let collection = match self
            .db
            .find_collection_by_uuid(
                Uuid::parse_str(cid).chain_err(|| "unable to parse collection uuid")?,
            )
            .chain_err(|| "unable to find collection to retrieve transactions for")?
        {
            Some(collection) => collection,
            None => return Ok(not_found("collection", cid, None)),
        };

        Ok(Object::list_of_transactions(
            self.db
                .find_transactions_for_collection(collection.id)
                .chain_err(|| "unable to find transactions by collection uuid")?,
        ))
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
                summary: &new.summary,
                notes: &new.notes,
                dated: &new.dated,
                disposition: &new.disposition,
                gain: &new.gain,
                loss: &new.loss,
                paid: new.paid,
            },
        ) {
            Ok(txn) => Ok(Object::Transaction(Transaction::from(txn))),
            _ => Ok(Object::fail("transaction-creation-failed")),
        }
    }

    pub fn retrieve_transaction(&self, cid: &str, tid: &str) -> Result<Object> {
        let collection = match self
            .db
            .find_collection_by_uuid(
                Uuid::parse_str(cid).chain_err(|| "unable to parse collection uuid")?,
            )
            .chain_err(|| "unable to find collection to retrieve transaction from")?
        {
            Some(collection) => collection,
            None => return Ok(not_found("collection", cid, None)),
        };

        match self
            .db
            .find_transaction_by_uuid(
                collection.id,
                Uuid::parse_str(tid).chain_err(|| "unable to parse transaction uuid")?,
            )
            .chain_err(|| "unable to find transaction by uuid")?
        {
            Some(transaction) => Ok(Object::Transaction(Transaction::from(transaction))),
            None => Ok(not_found("transaction", tid, None)),
        }
    }

    pub fn update_transaction(
        &self,
        cid: &str,
        tid: &str,
        upd: TransactionUpdateAttempt,
    ) -> Result<Object> {
        let collection = match self
            .db
            .find_collection_by_uuid(
                Uuid::parse_str(cid).chain_err(|| "unable to parse collection uuid")?,
            )
            .chain_err(|| "unable to find collection to update transaction for")?
        {
            Some(collection) => collection,
            None => return Ok(not_found("collection", cid, None)),
        };

        let transaction = match self
            .db
            .find_transaction_by_uuid(
                collection.id,
                Uuid::parse_str(tid).chain_err(|| "unable to parse transaction uuid")?,
            )
            .chain_err(|| "unable to find transaction to update")?
        {
            Some(transaction) => transaction,
            None => return Ok(not_found("transaction", tid, None)),
        };

        // update the transaction details, selectively
        match self.db.update_transaction(
            &transaction,
            db::UpdateTransaction {
                summary: upd.summary,
                notes: upd.notes,
                dated: upd.dated,
                disposition: upd.disposition,
                gain: upd.gain,
                loss: upd.loss,
                paid: Some(upd.paid),
            },
        ) {
            Ok(txn) => Ok(Object::Transaction(Transaction::from(txn))),
            _ => Ok(Object::fail("transaction-update-failed")),
        }
    }

    pub fn delete_transaction(&self, cid: &str, tid: &str) -> Result<Object> {
        let collection = match self
            .db
            .find_collection_by_uuid(
                Uuid::parse_str(cid).chain_err(|| "unable to parse collection uuid")?,
            )
            .chain_err(|| "unable to find collection to update transaction for")?
        {
            Some(collection) => collection,
            None => return Ok(not_found("collection", cid, None)),
        };

        let transaction = match self
            .db
            .find_transaction_by_uuid(
                collection.id,
                Uuid::parse_str(tid).chain_err(|| "unable to parse transaction uuid")?,
            )
            .chain_err(|| "unable to find transaction to update")?
        {
            Some(transaction) => transaction,
            None => return Ok(not_found("transaction", tid, None)),
        };

        match self.db.delete_transaction(transaction.id) {
            Ok(_) => Ok(Object::ok("transaction-removed")),
            _ => Ok(Object::fail("transaction-removal-failed")),
        }
    }

    pub fn create_goal(&self, uid: &str, new: GoalCreationAttempt) -> Result<Object> {
        let collector = match self
            .db
            .find_collector_by_uuid(
                Uuid::parse_str(uid).chain_err(|| "unable to parse collector uuid")?,
            )
            .chain_err(|| "unable to find collector to create goal for")?
        {
            Some(collector) => collector,
            None => return Ok(not_found("collector", uid, None)),
        };

        match self.db.create_goal(
            None,
            db::NewGoal {
                collector: collector.id,
                name: &new.name,
                ordinal: new.ordinal,
                target: &new.target,
                goal: &new.goal,
                total: new.total,
                progress: new.progress,
            },
        ) {
            Ok(goal) => Ok(Object::Goal(Goal::from(goal))),
            _ => Ok(Object::fail("goal-creation-failed")),
        }
    }

    pub fn update_goal(&self, uid: &str, gid: &str, upd: GoalUpdateAttempt) -> Result<Object> {
        let collector = match self
            .db
            .find_collector_by_uuid(
                Uuid::parse_str(uid).chain_err(|| "unable to parse collector uuid")?,
            )
            .chain_err(|| "unable to find collector to update goal for")?
        {
            Some(collector) => collector,
            None => return Ok(not_found("collector", uid, None)),
        };

        let goal = match self
            .db
            .find_goal_by_uuid(
                collector.id,
                Uuid::parse_str(gid).chain_err(|| "unable to parse goal uuid")?,
            )
            .chain_err(|| "unable to find goal to update")?
        {
            Some(goal) => goal,
            None => return Ok(not_found("goal", gid, None)),
        };

        match self.db.update_goal(
            &goal,
            db::UpdateGoal {
                name: upd.name,
                ordinal: upd.ordinal,
                target: upd.target,
                goal: upd.goal,
                total: upd.total,
                progress: upd.progress,
            },
        ) {
            Ok(goal) => Ok(Object::Goal(Goal::from(goal))),
            _ => Ok(Object::fail("goal-update-failed")),
        }
    }

    pub fn delete_goal(&self, uid: &str, gid: &str) -> Result<Object> {
        let collector = match self
            .db
            .find_collector_by_uuid(
                Uuid::parse_str(uid).chain_err(|| "unable to parse collector uuid")?,
            )
            .chain_err(|| "unable to find collector to update goal for")?
        {
            Some(collector) => collector,
            None => return Ok(not_found("collector", uid, None)),
        };

        let goal = match self
            .db
            .find_goal_by_uuid(
                collector.id,
                Uuid::parse_str(gid).chain_err(|| "unable to parse goal uuid")?,
            )
            .chain_err(|| "unable to find goal to update")?
        {
            Some(goal) => goal,
            None => return Ok(Object::ok("goal-already-gone")),
        };

        match self.db.delete_goal(goal.id) {
            Ok(_) => Ok(Object::ok("goal-removed")),
            _ => Ok(Object::fail("goal-removal-failed")),
        }
    }

    pub fn retrieve_goals_for_collector(&self, uid: &str) -> Result<Object> {
        let collector = match self
            .db
            .find_collector_by_uuid(
                Uuid::parse_str(uid).chain_err(|| "unable to parse collector uuid")?,
            )
            .chain_err(|| "unable to find collector to retrieve goals from")?
        {
            Some(collector) => collector,
            None => return Ok(not_found("collector", uid, None)),
        };

        Ok(Object::list_of_goals(
            self.db
                .find_goals_for_collector(collector.id)
                .chain_err(|| "unable to find goals by collector uuid")?,
        ))
    }

    pub fn retrieve_goal(&self, uid: &str, gid: &str) -> Result<Object> {
        let collector = match self
            .db
            .find_collector_by_uuid(
                Uuid::parse_str(uid).chain_err(|| "unable to parse collector uuid")?,
            )
            .chain_err(|| "unable to find collector to retrieve goal for")?
        {
            Some(collector) => collector,
            None => return Ok(not_found("collector", uid, None)),
        };

        match self
            .db
            .find_goal_by_uuid(
                collector.id,
                Uuid::parse_str(gid).chain_err(|| "unable to parse goal uuid")?,
            )
            .chain_err(|| "unable to find goal by uuid")?
        {
            Some(goal) => Ok(Object::Goal(Goal::from(goal))),
            None => Ok(not_found("goal", gid, None)),
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
                code: &new.code,
                description: &new.description,
                main: &new.main,
                side: &new.side,
                maybe: &new.maybe,
            },
        ) {
            Ok(deck) => Ok(Object::Deck(Deck::from(deck))),
            _ => Ok(Object::fail("deck-creation-failed")),
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

        let deck = match self
            .db
            .find_deck_by_uuid(
                collector.id,
                Uuid::parse_str(did).chain_err(|| "unable to parse deck uuid")?,
            )
            .chain_err(|| "unable to find deck to update")?
        {
            Some(deck) => deck,
            None => return Ok(not_found("deck", did, None)),
        };

        match self.db.update_deck(
            &deck,
            db::UpdateDeck {
                title: upd.title,
                code: upd.code,
                description: upd.description,
                main: upd.main,
                side: upd.side,
                maybe: upd.maybe,
            },
        ) {
            Ok(deck) => Ok(Object::Deck(Deck::from(deck))),
            _ => Ok(Object::fail("deck-update-failed")),
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

        let deck = match self
            .db
            .find_deck_by_uuid(
                collector.id,
                Uuid::parse_str(did).chain_err(|| "unable to parse deck uuid")?,
            )
            .chain_err(|| "unable to find deck to update")?
        {
            Some(deck) => deck,
            None => return Ok(Object::ok("deck-already-gone")),
        };

        match self.db.delete_deck(deck.id) {
            Ok(_) => Ok(Object::ok("deck-removed")),
            _ => Ok(Object::fail("deck-removal-failed")),
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

        Ok(Object::list_of_decks(
            self.db
                .find_decks_for_collector(collector.id)
                .chain_err(|| "unable to find decks by collector uuid")?,
        ))
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
