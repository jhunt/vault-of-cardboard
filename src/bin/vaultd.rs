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

fn main() {
    let mut router = Router::new();

    router.post(
        "/v1/authenticate",
        |r: &mut Request| {
            let api = boot();
            match serde_json::from_reader(&mut r.body) {
                Err(e) => {
                    println!("error: {}", e);
                    Ok(Response::with((status::BadRequest, "bad request")))
                }
                Ok(attempt) => match api.authenticate(attempt) {
                    Ok(res) => Ok(Response::with((status::Ok, json!(res).to_string()))),
                    Err(e) => {
                        println!("authn fail: {}", e);
                        Ok(Response::with((status::Forbidden, "authn failed")))
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
                    Ok(Response::with((status::BadRequest, "bad request")))
                }
                Ok(attempt) => match api.signup(attempt) {
                    Ok(res) => Ok(Response::with((status::Ok, json!(res).to_string()))),
                    Err(e) => {
                        println!("authn fail: {}", e);
                        Ok(Response::with((status::Forbidden, "authn failed")))
                    }
                },
            }
        },
        "v1_signup_handler",
    );

    router.get(
        "/v1/collectors/:uid/decks",
        |r: &mut Request| {
            let api = boot();
            Ok(Response::with((status::Ok, "hi")))
        },
        "v1_get_all_transactions_handler",
    );

    router.post(
        "/v1/collectors/:uid/decks",
        |r: &mut Request| {
            let api = boot();
            let uid = r.extensions.get::<Router>().unwrap().find("uid").unwrap();
            match serde_json::from_reader(&mut r.body) {
                Err(e) => {
                    println!("error: {}", e);
                    Ok(Response::with((status::BadRequest, "bad request")))
                }
                Ok(attempt) => match api.post_transaction(uid, attempt) {
                    Ok(res) => Ok(Response::with((status::Ok, json!(res).to_string()))),
                    Err(e) => {
                        println!("transaction fail: {}", e);
                        Ok(Response::with((status::Forbidden, "transaction failed")))
                    }
                },
            }
        },
        "v1_post_new_transaction_handler",
    );

    router.get(
        "/v1/collectors/:uid/decks/:tid",
        |r: &mut Request| {
            let api = boot();
            Ok(Response::with((status::Ok, "hi")))
        },
        "v1_get_single_transaction_handler",
    );

    router.patch(
        "/v1/collectors/:uid/decks/:tid",
        |r: &mut Request| {
            let api = boot();
            Ok(Response::with((status::Ok, "hi")))
        },
        "v1_update_single_transaction_handler",
    );

    router.delete(
        "/v1/collectors/:uid/decks/:tid",
        |r: &mut Request| {
            let api = boot();
            Ok(Response::with((status::Ok, "hi")))
        },
        "v1_delete_single_transaction_handler",
    );

    router.get(
        "/v1/collectors/:uid/decks",
        |r: &mut Request| {
            let api = boot();
            Ok(Response::with((status::Ok, "hi")))
        },
        "v1_get_all_decks_handler",
    );

    router.post(
        "/v1/collectors/:uid/decks",
        |r: &mut Request| {
            let api = boot();
            let uid = r.extensions.get::<Router>().unwrap().find("uid").unwrap();
            match serde_json::from_reader(&mut r.body) {
                Err(e) => {
                    println!("error: {}", e);
                    Ok(Response::with((status::BadRequest, "bad request")))
                }
                Ok(attempt) => match api.post_deck(uid, attempt) {
                    Ok(res) => Ok(Response::with((status::Ok, json!(res).to_string()))),
                    Err(e) => {
                        println!("deck fail: {}", e);
                        Ok(Response::with((status::Forbidden, "deck failed")))
                    }
                },
            }
        },
        "v1_post_new_deck_handler",
    );

    router.get(
        "/v1/collectors/:uid/decks/:did",
        |r: &mut Request| {
            let api = boot();
            Ok(Response::with((status::Ok, "hi")))
        },
        "v1_get_single_deck_handler",
    );

    router.patch(
        "/v1/collectors/:uid/decks/:tid",
        |r: &mut Request| {
            let api = boot();
            Ok(Response::with((status::Ok, "hi")))
        },
        "v1_update_single_deck_handler",
    );

    router.delete(
        "/v1/collectors/:uid/decks/:did",
        |r: &mut Request| {
            let api = boot();
            Ok(Response::with((status::Ok, "hi")))
        },
        "v1_delete_single_deck_handler",
    );

    let bind = "localhost:3000";
    println!("example-web starting up on {}", bind);
    Iron::new(router).http(bind).unwrap();
}
