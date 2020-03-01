use iron::prelude::*;
use iron::status;
use router::Router;
use serde_json::json;
use std::env;
use std::path::Path;

use vault_of_cardboard::api::{Object, API};
use vault_of_cardboard::db::Database;

fn boot() -> API {
    API::new(
        Database::connect(
            &env::var("VCB_DATABASE_URL").expect("VCB_DATABASE_URL must be set in environment"),
            &env::var("VCB_REDIS_URL").expect("VCB_REDIS_URL must be set in environment"),
            &Path::new(&env::var("VCB_FS_ROOT").expect("VCB_FS_ROOT must be set in environment")),
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
            None => return done!(400 => "bad request"),
        }
    };
}

macro_rules! done {
    (204) => {
        Ok(Response::with(status::NoContent))
    };

    (200 => $o: expr) => {
        Ok(Response::with((
            status::Ok,
            format!("{}\n", json!($o).to_string()),
        )))
    };

    (400 => $s: expr) => {
        Ok(Response::with((status::BadRequest, format!("{}\n", $s))))
    };

    (403 => $s: expr) => {
        Ok(Response::with((status::Forbidden, format!("{}\n", $s))))
    };

    (500 => $s: expr) => {
        Ok(Response::with((
            status::InternalServerError,
            format!("{}\n", $s),
        )))
    };

    (??? => $s: expr) => {
        Ok(Response::with((
            status::InternalServerError,
            format!("{}: unimplemented.\n", $s),
        )))
    };

    ($object: expr) => {
        Ok(Response::with((
            match &$object {
                Object::NotFound(_) => status::NotFound,
                _ => status::Ok,
            },
            format!("{}\n", json!(&$object).to_string()),
        )))
    };
}

fn main() {
    let mut router = Router::new();

    router.get(
        "/cards.json",
        |_: &mut Request| {
            let api = boot();
            match api.file("cards.json") {
                Ok(f) => Ok(Response::with((status::Ok, f))),
                Err(e) => {
                    println!("error: {}", e);
                    done!(500 => "internal server error")
                },
            }
        },
        "cards_json_file",
    );

    router.get(
        "/prices.json",
        |_: &mut Request| {
            let api = boot();
            match api.file("prices.json") {
                Ok(f) => Ok(Response::with((status::Ok, f))),
                Err(e) => {
                    println!("error: {}", e);
                    done!(500 => "internal server error")
                },
            }
        },
        "prices_json_file",
    );

    router.get(
        "/collectors/:uid/collections/_/collection.json",
        |r: &mut Request| {
            let api = boot();
            let uid = param!(r, "uid");
            match api.file(&format!("c/{}/_/collection.json", uid)) {
                Ok(f) => Ok(Response::with((status::Ok, f))),
                Err(e) => {
                    println!("error: {}", e);
                    done!(500 => "internal server error")
                },
            }
        },
        "default_collection_json_file",
    );

    router.post(
        "/v1/authenticate",
        |r: &mut Request| {
            let api = boot();
            match serde_json::from_reader(&mut r.body) {
                Err(e) => {
                    println!("error: {}", e);
                    done!(400 => "bad request")
                },
                Ok(attempt) => match api.authenticate(attempt) {
                    Ok(res) => done!(res),
                    Err(e) => {
                        println!("authn fail: {}", e);
                        done!(403 => "authentication failed")
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
                    done!(400 => "bad request")
                }
                Ok(attempt) => match api.signup(attempt) {
                    Ok(res) => done!(res),
                    Err(e) => {
                        println!("authn fail: {}", e);
                        done!(403 => "authentication failed")
                    }
                },
            }
        },
        "v1_signup_handler",
    );

    router.get(
        "/v1/collectors/:uid/collections/_/transactions",
        |r: &mut Request| {
            let api = boot();
            let uid = param!(r, "uid");

            match api.retrieve_transactions_for_collection(&uid) {
                Ok(res) => done!(res),
                Err(e) => {
                    println!("transactions retrieval fail: {}", e);
                    done!(500 => "transactions retrieval failed")
                }
            }
        },
        "v1_get_all_transactions_handler",
    );

    router.post(
        "/v1/collectors/:uid/collections/_/transactions",
        |r: &mut Request| {
            let api = boot();
            let uid = param!(r, "uid");

            match serde_json::from_reader(&mut r.body) {
                Err(e) => {
                    println!("error: {}", e);
                    done!(400 => "bad request")
                }
                Ok(attempt) => match api.post_transaction(&uid, attempt) {
                    Ok(res) => done!(res),
                    Err(e) => {
                        println!("transaction fail: {}", e);
                        done!(403 => "transaction creation failed")
                    }
                },
            }
        },
        "v1_post_new_transaction_handler",
    );

    router.get(
        "/v1/collectors/:uid/collections/_/transactions/:tid",
        |r: &mut Request| {
            let api = boot();
            let uid = param!(r, "uid");
            let tid = param!(r, "tid");

            match api.retrieve_transaction(&uid, &tid) {
                Ok(res) => done!(res),
                Err(e) => {
                    println!("transaction retrieval fail: {}", e);
                    done!(500 => "transaction retrieval failed")
                }
            }
        },
        "v1_get_single_transaction_handler",
    );

    router.patch(
        "/v1/collectors/:uid/collections/_/transactions/:tid",
        |r: &mut Request| {
            let api = boot();
            let uid = param!(r, "uid");
            let tid = param!(r, "tid");

            match serde_json::from_reader(&mut r.body) {
                Err(e) => {
                    println!("error: {}", e);
                    done!(400 => "bad request")
                }
                Ok(attempt) => match api.update_transaction(&uid, &tid, attempt) {
                    Ok(res) => done!(res),
                    Err(e) => {
                        println!("transaction update fail: {}", e);
                        done!(500 => "transaction update failed")
                    }
                },
            }
        },
        "v1_update_single_transaction_handler",
    );

    router.delete(
        "/v1/collectors/:uid/collections/_/transactions/:tid",
        |r: &mut Request| {
            let api = boot();
            let uid = param!(r, "uid");
            let tid = param!(r, "tid");

            match api.delete_transaction(&uid, &tid) {
                Ok(res) => done!(res),
                Err(e) => {
                    println!("transaction removal fail: {}", e);
                    done!(500 => "transaction removal failed")
                }
            }
        },
        "v1_delete_single_transaction_handler",
    );

    router.get(
        "/v1/collectors/:uid/decks",
        |r: &mut Request| {
            let api = boot();
            let uid = param!(r, "uid");

            match api.retrieve_decks_for_collector(&uid) {
                Ok(res) => done!(res),
                Err(e) => {
                    println!("decks retrieval fail: {}", e);
                    done!(500 => "decks retrieval failed")
                }
            }
        },
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
                    done!(400 => "bad request")
                }
                Ok(attempt) => match api.create_deck(&uid, attempt) {
                    Ok(res) => done!(res),
                    Err(e) => {
                        println!("deck fail: {}", e);
                        done!(500 => "deck creation failed")
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
            let uid = param!(r, "uid");
            let did = param!(r, "did");

            match api.retrieve_deck(&uid, &did) {
                Ok(res) => done!(res),
                Err(e) => {
                    println!("deck retrieval fail: {}", e);
                    done!(500 => "deck retrieval failed")
                }
            }
        },
        "v1_get_single_deck_handler",
    );

    router.patch(
        "/v1/collectors/:uid/decks/:did",
        |r: &mut Request| {
            let api = boot();
            let uid = param!(r, "uid");
            let did = param!(r, "did");

            match serde_json::from_reader(&mut r.body) {
                Err(e) => {
                    println!("error: {}", e);
                    done!(400 => "bad request")
                }
                Ok(attempt) => match api.update_deck(&uid, &did, attempt) {
                    Ok(res) => done!(res),
                    Err(e) => {
                        println!("deck update fail: {}", e);
                        done!(500 => "deck update failed")
                    }
                },
            }
        },
        "v1_update_single_deck_handler",
    );

    router.delete(
        "/v1/collectors/:uid/decks/:did",
        |r: &mut Request| {
            let api = boot();
            let uid = param!(r, "uid");
            let did = param!(r, "did");

            match api.delete_deck(&uid, &did) {
                Ok(res) => done!(res),
                Err(e) => {
                    println!("deck removal fail: {}", e);
                    done!(500 => "deck removal failed")
                }
            }
        },
        "v1_delete_single_deck_handler",
    );

    let bind = match env::var("VCB_LISTEN") {
        Ok(v) => v,
        Err(_) => "localhost:3000".to_string(),
    };
    println!("vault-of-cardboard starting up on {}", bind);
    Iron::new(router).http(bind).unwrap();
}
