use iron::headers::ContentType;
use iron::mime::{Mime, SubLevel, TopLevel};
use iron::prelude::*;
use iron::status;
use router::Router;
use serde_json::json;
use std::env;
use std::path::Path;

header! { (WwwAuthenticate, "WWW-Authenticate") => [String] }

use crate::api::{Object, API};
use crate::db::Database;

fn boot() -> API {
    let idle = 3600;
    let idle = match env::var("VCB_SESSION_IDLE") {
        Ok(v) => match v.parse::<u32>() {
            Ok(v) => v,
            Err(_) => idle,
        },
        Err(_) => idle,
    };
    API::new(
        Database::connect(
            &env::var("VCB_DATABASE_URL").expect("VCB_DATABASE_URL must be set in environment"),
            &env::var("VCB_REDIS_URL").expect("VCB_REDIS_URL must be set in environment"),
            &Path::new(&env::var("VCB_FS_ROOT").expect("VCB_FS_ROOT must be set in environment")),
            idle,
        )
        .unwrap(),
    )
}

fn auth(r: &Request) -> Option<String> {
    use iron::headers::{Authorization, Basic};

    match r.headers.get::<Authorization<Basic>>() {
        Some(v) => Some(v.username.to_string()),
        None => None,
    }
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

macro_rules! auth {
    ($api: expr, $r: expr, $uid: expr) => {
        match $api.guard(auth($r), &$uid) {
            Some(401) => return done!(401 => "authentication required"),
            Some(403) => return done!(403 => "forbidden"),
            _ => (),
        };
    };
}

fn json_response(code: status::Status, json: String) -> Response {
    let mut r = Response::with((code, format!("{}\n", json)));
    r.headers.set(ContentType(Mime(
        TopLevel::Application,
        SubLevel::Json,
        vec![],
    )));
    r
}

macro_rules! done {
    (204) => {
        Ok(Response::with(status::NoContent))
    };

    (200 => $o: expr) => {
        Ok(json_response(status::Ok, json!($o).to_string()))
    };

    (400 => $s: expr) => {
        Ok(Response::with((status::BadRequest, format!("{}\n", $s))))
    };

    (401 => $s: expr) => {{
        let mut r = Response::with((status::Unauthorized, format!("{}\n", $s)));
        r.headers.set(WwwAuthenticate(
            r#"Basic realm="vaultofcardboard.com""#.to_owned(),
        ));
        Ok(r)
    }};

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
        Ok(json_response(
            match &$object {
                Object::NotFound(_) => status::NotFound,
                _ => status::Ok,
            },
            json!(&$object).to_string(),
        ))
    };
}

pub fn run() {
    let mut router = Router::new();

    router.get(
        "/cards.json",
        |_: &mut Request| {
            let api = boot();
            match api.file("cards.json") {
                Ok(f) => {
                    let mut r = Response::with((status::Ok, f));
                    r.headers.set(ContentType(Mime(
                        TopLevel::Application,
                        SubLevel::Json,
                        vec![],
                    )));
                    Ok(r)
                }
                Err(e) => {
                    println!("error: {}", e);
                    done!(500 => "internal server error")
                }
            }
        },
        "cards_json_file",
    );

    router.get(
        "/prices.json",
        |_: &mut Request| {
            let api = boot();
            match api.file("prices.json") {
                Ok(f) => {
                    let mut r = Response::with((status::Ok, f));
                    r.headers.set(ContentType(Mime(
                        TopLevel::Application,
                        SubLevel::Json,
                        vec![],
                    )));
                    Ok(r)
                }
                Err(e) => {
                    println!("error: {}", e);
                    done!(500 => "internal server error")
                }
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
                Ok(f) => {
                    let mut r = Response::with((status::Ok, f));
                    r.headers.set(ContentType(Mime(
                        TopLevel::Application,
                        SubLevel::Json,
                        vec![],
                    )));
                    Ok(r)
                }
                Err(e) => {
                    println!("error: {}", e);
                    done!(500 => "internal server error")
                }
            }
        },
        "default_collection_json_file",
    );

    router.post(
        "/v1/whoami",
        |r: &mut Request| {
            let api = boot();
            match serde_json::from_reader(&mut r.body) {
                Err(e) => {
                    println!("error: {}", e);
                    done!(400 => "bad request")
                }
                Ok(who) => {
                    let r = api.whoami(who);
                    done!(r)
                }
            }
        },
        "v1_whoami_handler",
    );

    router.post(
        "/v1/authenticate",
        |r: &mut Request| {
            let api = boot();
            match serde_json::from_reader(&mut r.body) {
                Err(e) => {
                    println!("error: {}", e);
                    done!(400 => "bad request")
                }
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
            auth!(api, r, &uid);

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
            auth!(api, r, &uid);

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
            auth!(api, r, &uid);

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
        "/v1/collectors/:uid/goals",
        |r: &mut Request| {
            let api = boot();
            let uid = param!(r, "uid");

            match api.retrieve_goals_for_collector(&uid) {
                Ok(res) => done!(res),
                Err(e) => {
                    println!("goals retrieval fail: {}", e);
                    done!(500 => "goals retrieval failed")
                }
            }
        },
        "v1_get_all_goals_handler",
    );

    router.post(
        "/v1/collectors/:uid/goals",
        |r: &mut Request| {
            let api = boot();
            let uid = param!(r, "uid");
            auth!(api, r, &uid);

            match serde_json::from_reader(&mut r.body) {
                Err(e) => {
                    println!("error: {}", e);
                    done!(400 => "bad request")
                }
                Ok(attempt) => match api.create_goal(&uid, attempt) {
                    Ok(res) => done!(res),
                    Err(e) => {
                        println!("goal fail: {}", e);
                        done!(500 => "goal creation failed")
                    }
                },
            }
        },
        "v1_post_new_goal_handler",
    );

    router.get(
        "/v1/collectors/:uid/goals/:gid",
        |r: &mut Request| {
            let api = boot();
            let uid = param!(r, "uid");
            let gid = param!(r, "gid");

            match api.retrieve_goal(&uid, &gid) {
                Ok(res) => done!(res),
                Err(e) => {
                    println!("goal retrieval fail: {}", e);
                    done!(500 => "goal retrieval failed")
                }
            }
        },
        "v1_get_single_goal_handler",
    );

    router.patch(
        "/v1/collectors/:uid/goals/:gid",
        |r: &mut Request| {
            let api = boot();
            let uid = param!(r, "uid");
            let gid = param!(r, "gid");
            auth!(api, r, &uid);

            match serde_json::from_reader(&mut r.body) {
                Err(e) => {
                    println!("error: {}", e);
                    done!(400 => "bad request")
                }
                Ok(attempt) => match api.update_goal(&uid, &gid, attempt) {
                    Ok(res) => done!(res),
                    Err(e) => {
                        println!("goal update fail: {}", e);
                        done!(500 => "goal update failed")
                    }
                },
            }
        },
        "v1_update_single_goal_handler",
    );

    router.delete(
        "/v1/collectors/:uid/goals/:gid",
        |r: &mut Request| {
            let api = boot();
            let uid = param!(r, "uid");
            let gid = param!(r, "gid");
            auth!(api, r, &uid);

            match api.delete_goal(&uid, &gid) {
                Ok(res) => done!(res),
                Err(e) => {
                    println!("goal removal fail: {}", e);
                    done!(500 => "goal removal failed")
                }
            }
        },
        "v1_delete_single_goal_handler",
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
            auth!(api, r, &uid);

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
            auth!(api, r, &uid);

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
            auth!(api, r, &uid);

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
