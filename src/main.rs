extern crate core;
extern crate rocket;
use core::rest::rest::ParsecConfig;
use mongodb::results::InsertOneResult;
use mongodb::{options::ClientOptions, Client};
use rocket::serde::json::Json;
use rocket::State;
use rocket::*;
use std::env;
async fn init_mongo() -> mongodb::Client {
    let mongo_uri = env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client_options = ClientOptions::parse(&mongo_uri).await.unwrap();
    Client::with_options(client_options).unwrap()
}

#[rocket::launch]
async fn rocket() -> _ {
    let client = init_mongo().await;
    rocket::build()
        .manage(client)
        .mount("/", routes![hello_world])
}
// POST /parsec
/// Receives a JSON body matching `ParsecConfig`
/// Inserts into MongoDB, returns the inserted_id
#[post("/parsec", format = "json", data = "<config>")]
async fn create_parsec_config(
    mongo: &State<Client>,
    config: Json<ParsecConfig>,
) -> Result<Json<InsertOneResult>, rocket::http::Status> {
    // Choose your database and collection name:
    let db = mongo.database("my_database");
    let collection = db.collection::<ParsecConfig>("parsec_configs");

    // Insert into MongoDB
    let insert_result = collection
        .insert_one(config.into_inner(), None)
        .await
        .map_err(|e| {
            eprintln!("Mongo insert error: {}", e);
            rocket::http::Status::InternalServerError
        })?;

    // Return the `InsertOneResult` as JSON
    Ok(Json(insert_result))
}
#[get("/")]
fn hello_world() -> &'static str {
    "Hello, world7!"
}
