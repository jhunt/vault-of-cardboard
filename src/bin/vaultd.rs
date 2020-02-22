use iron::prelude::*;
use iron::status;
use router::Router;
use serde_json::json;
use std::env;

use vault_of_cardboard::api::API;
use vault_of_cardboard::db::Database;

fn boot() -> API {
    API::new(
        Database::connect(
            &env::var("VCB_DATABASE_URL").expect("VCB_DATABASE_URL must be set in environment"),
            &env::var("VCB_REDIS_URL").expect("VCB_REDIS_URL must be set in environment"),
        )
        .unwrap(),
    )
}

fn param(r: &Request, name: &str) -> Option<String> {
    match r.extensions.get::<Router>() {
        None => None,
        Some(params) => match params.find(name) {
            None => None,
            Some(v) => Some(v.to_string()),
        },
    }
}

macro_rules! param {
    ($r: expr, $e: expr) => {
        match param($r, $e) {
            Some(v) => v,
            None => return bad_request!("bad request"),
        }
    };
}

macro_rules! bad_request {
    ($s: expr) => {
        Ok(Response::with((status::BadRequest, $s)))
    };
}

macro_rules! okay {
    ($o: expr) => {
        Ok(Response::with((status::Ok, json!($o).to_string())))
    };
}

macro_rules! forbidden {
    ($s: expr) => {
        Ok(Response::with((status::Forbidden, $s)))
    };
}

macro_rules! unimpl {
    ($s: expr) => {
        Ok(Response::with((
            status::InternalServerError,
            format!("{}: unimplemented.", $s),
        )))
    };
}

fn main() {
    let mut router = Router::new();

    router.post(
        "/v1/authenticate",
        |r: &mut Request| {
            let api = boot();
            match serde_json::from_reader(&mut r.body) {
                Err(e) => {
                    println!("error: {}", e);
                    bad_request!("bad request")
                }
                Ok(attempt) => match api.authenticate(attempt) {
                    Ok(res) => okay!(res),
                    Err(e) => {
                        println!("authn fail: {}", e);
                        forbidden!("authentication failed")
                    }
                },
            }
        },
        "v1_authenticate_handler",
    );

    router.post(
        "/v1/signup",
        |r: &mut Request| {
            let api = boot();
            match serde_json::from_reader(&mut r.body) {
                Err(e) => {
                    println!("error: {}", e);
                    bad_request!("bad request")
                }
                Ok(attempt) => match api.signup(attempt) {
                    Ok(res) => okay!(res),
                    Err(e) => {
                        println!("authn fail: {}", e);
                        forbidden!("authentication failed")
                    }
                },
            }
        },
        "v1_signup_handler",
    );

    router.get(
        "/v1/collectors/:uid/decks",
        |_r: &mut Request| unimpl!("deck retrieval"),
        "v1_get_all_transactions_handler",
    );

    router.post(
        "/v1/collectors/:uid/decks",
        |r: &mut Request| {
            let api = boot();
            let uid = param!(r, "uid");
            match serde_json::from_reader(&mut r.body) {
                Err(e) => {
                    println!("error: {}", e);
                    bad_request!("bad request")
                }
                Ok(attempt) => match api.post_transaction(&uid, attempt) {
                    Ok(res) => okay!(res),
                    Err(e) => {
                        println!("transaction fail: {}", e);
                        forbidden!("transaction creation failed")
                    }
                },
            }
        },
        "v1_post_new_transaction_handler",
    );

    router.get(
        "/v1/collectors/:uid/transactions/:tid",
        |_r: &mut Request| unimpl!("transaction retrieval"),
        "v1_get_single_transaction_handler",
    );

    router.patch(
        "/v1/collectors/:uid/transactions/:tid",
        |_r: &mut Request| unimpl!("transaction update"),
        "v1_update_single_transaction_handler",
    );

    router.delete(
        "/v1/collectors/:uid/transactions/:tid",
        |_r: &mut Request| unimpl!("transaction removal"),
        "v1_delete_single_transaction_handler",
    );

    router.get(
        "/v1/collectors/:uid/decks",
        |_r: &mut Request| unimpl!("deck retrieval"),
        "v1_get_all_decks_handler",
    );

    router.post(
        "/v1/collectors/:uid/decks",
        |r: &mut Request| {
            let api = boot();
            let uid = param!(r, "uid");
            match serde_json::from_reader(&mut r.body) {
                Err(e) => {
                    println!("error: {}", e);
                    bad_request!("bad request")
                }
                Ok(attempt) => match api.post_deck(&uid, attempt) {
                    Ok(res) => okay!(res),
                    Err(e) => {
                        println!("deck fail: {}", e);
                        forbidden!("deck creation failed")
                    }
                },
            }
        },
        "v1_post_new_deck_handler",
    );

    router.get(
        "/v1/collectors/:uid/decks/:did",
        |_r: &mut Request| unimpl!("deck retrevial"),
        "v1_get_single_deck_handler",
    );

    router.patch(
        "/v1/collectors/:uid/decks/:tid",
        |_r: &mut Request| unimpl!("deck update"),
        "v1_update_single_deck_handler",
    );

    router.delete(
        "/v1/collectors/:uid/decks/:did",
        |_r: &mut Request| unimpl!("deck removal"),
        "v1_delete_single_deck_handler",
    );

    let bind = match env::var("VCB_LISTEN") {
        Ok(v) => v,
        Err(_) => "localhost:3000".to_string(),
    };
    println!("vault-of-cardboard starting up on {}", bind);
    Iron::new(router).http(bind).unwrap();
}
