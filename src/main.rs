#[macro_use]
extern crate rocket;
extern crate core;
use mongodb::{options::ClientOptions, Client};
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

#[get("/")]
fn hello_world() -> &'static str {
    "Hello, world3!"
}
