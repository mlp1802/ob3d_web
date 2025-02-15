pub mod dao;

extern crate core;
extern crate rocket;
use core::rest::rest::ParsecConfig;
use dao::ParsecDao;
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
    let client: Client = init_mongo().await;
    let db = client.database("ob3d_database");

    let parsec_dao = ParsecDao::new(db);
    rocket::build()
        .manage(parsec_dao)
        .mount("/", routes![hello_world])
}

pub fn handle_insert_one(
    obj: Option<InsertOneResult>,
) -> Result<Json<InsertOneResult>, rocket::http::Status> {
    match obj {
        Some(obj) => Result::Ok(Json(obj)),
        None => Result::Err(rocket::http::Status::InternalServerError),
    }
}
#[post("/parsec/update", format = "json", data = "<config>")]
async fn create_parsec_config(
    dao: &State<ParsecDao>,
    config: Json<ParsecConfig>,
) -> Result<Json<InsertOneResult>, rocket::http::Status> {
    let result = dao.insert(config.into_inner()).await;
    handle_insert_one(result)
}

#[get("/")]
fn hello_world() -> &'static str {
    "Hello, world7!"
}

// POST /parsec
// Receives a JSON body matching `ParsecConfig`
// Inserts into MongoDB, returns the inserted_id
//#[post("/parsec", format = "json", data = "<config>")]
//async fn example(
//    mongo: &State<Client>,
//    config: Json<ParsecConfig>,
//) -> Result<Json<InsertOneResult>, rocket::http::Status> {
//    // Choose your database and collection name:
//    let db = mongo.database("my_database");
//    let collection = db.collection::<ParsecConfig>("parsec_configs");
//    let a: ParsecConfig = config.into_inner();
//    // Insert into MongoDB
//    let insert_result = collection.insert_one(a, None).await.map_err(|e| {
//        eprintln!("Mongo insert error: {}", e);
//        rocket::http::Status::InternalServerError
//    })?;
//
//    // Return the `InsertOneResult` as JSON
//    Ok(Json(insert_result))
//}
