pub mod dao;

extern crate core;
extern crate rocket;
use core::parsec::parsec_controls::ParsecControls;
use core::pvp_settings::ParsecConfig;
use dao::ParsecDao;
use http::Status;
use mongodb::results::InsertOneResult;
use mongodb::{options::ClientOptions, Client};
use rocket::serde::json::Json;
use rocket::State;
use rocket::*;
use serde::Serialize;
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
        .mount("/", routes![create_parsec_config, get_parsec_controls])
}

pub fn handle_insert_one_empty_result(
    result: Option<InsertOneResult>,
) -> Result<(), rocket::http::Status> {
    match result {
        Some(_) => Ok(()),
        None => Result::Err(rocket::http::Status::InternalServerError),
    }
}
pub fn handle_get<T>(result: Option<T>) -> Result<Json<T>, rocket::http::Status>
where
    T: Serialize,
{
    match result {
        Some(result) => Ok(Json(result)),
        None => Result::Err(rocket::http::Status::InternalServerError),
    }
}

#[post("/parsec/config", format = "json", data = "<config>")]
async fn create_parsec_config(
    dao: &State<ParsecDao>,
    config: Json<ParsecControls>,
) -> Result<(), Status> {
    let result = dao.insert_parsec_config(config.into_inner()).await;
    handle_insert_one_empty_result(result)
}
#[get("/parsec/config/<id>")]
async fn get_parsec_controls(
    dao: &State<ParsecDao>,
    id: &str,
) -> Result<Json<ParsecControls>, Status> {
    let result = dao.get_parsec_controls(id).await;
    handle_get(result)
}
