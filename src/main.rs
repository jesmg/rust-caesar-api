#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
struct CaesarInput<'a> {
    secret: &'a str,
    shift: u8
}

#[derive(Serialize)]
struct Output {
    secret: String
}

#[post("/caesar/encrypt", format = "json", data = "<body>")]
fn caesar_encrypt(body: Json<CaesarInput>) -> Result<Json<Output>, JsonValue> {
    match cipher_api::encrypt(body.secret, &body.shift) {
        Ok(v) => Ok(Json(Output {
            secret: v
        })),
        Err(e) => Err(json!({
            "status": "error",
            "reason": e 
        })),
    }
}

#[post("/caesar/decrypt", format = "json", data = "<body>")]
fn caesar_decrypt(body: Json<CaesarInput>) -> Result<Json<Output>, JsonValue> {
    match cipher_api::decrypt(body.secret, &body.shift) {
        Ok(v) => Ok(Json(Output {
            secret: v
        })),
        Err(e) => Err(json!({
            "status": "error",
            "reason": e 
        })),
    }
}

#[get("/liveness")]
fn liveness() -> JsonValue {
    json!({
        "status": "ok"
    })
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "resource was not found"
    })
}

fn main() {
    rocket::ignite()
        .mount("/", routes![caesar_encrypt, caesar_decrypt, liveness])
        .register(catchers![not_found])
        .launch();
}