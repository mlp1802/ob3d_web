pub mod custom_styles_dao;
pub mod dao;
pub mod file_sync;
pub mod parsec_dao;

extern crate core;
extern crate rocket;
use core::custom_styles::custom_style::CustomStyle;
use core::custom_styles::serial_custom_style::SerialCustomStyle;
use core::parsec::parsec_controls::ParsecControls;
use core::pvp_settings::ParsecConfig;
use core::rest::syncher::SynchRequest;
use custom_styles_dao::CustomStylesDao;
use http::Status;
use mongodb::results::{InsertOneResult, UpdateResult};
use mongodb::{options::ClientOptions, Client};
use parsec_dao::ParsecDao;
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::State;
use rocket::*;
use serde::Serialize;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
async fn init_mongo() -> mongodb::Client {
    let mongo_uri = env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client_options = ClientOptions::parse(&mongo_uri).await.unwrap();
    Client::with_options(client_options).unwrap()
}

#[rocket::launch]
async fn rocket() -> _ {
    let client: Client = init_mongo().await;
    let db = client.database("ob3d_database");

    let parsec_dao = ParsecDao::new(db.clone());
    let custom_styles_dao = CustomStylesDao::new(db.clone());
    rocket::build()
        .manage(parsec_dao)
        .manage(custom_styles_dao)
        .mount(
            "/",
            routes![
                create_parsec_config,
                get_parsec_controls,
                create_custom_style,
                get_custom_style,
                file_sync::sync_check,
                file_sync::download
            ],
        )
}

pub fn handle_insert_one_empty_result(
    result: Option<UpdateResult>,
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
    println!("Create Parsec Config {}", config.parsec_id);
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

#[post("/custom_styles", format = "json", data = "<config>")]
async fn create_custom_style(
    dao: &State<CustomStylesDao>,
    config: Json<SerialCustomStyle>,
) -> Result<(), Status> {
    println!("Create custom style {}", config.id);
    let result = dao.insert_custom_style(config.into_inner()).await;
    handle_insert_one_empty_result(result)
}
#[get("/custom_styles/<id>")]
async fn get_custom_style(
    dao: &State<CustomStylesDao>,
    id: &str,
) -> Result<Json<SerialCustomStyle>, Status> {
    let result = dao.get_custom_style(id).await;
    handle_get(result)
}
