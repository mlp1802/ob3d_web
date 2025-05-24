use core::parsec::parsec_controls::ParsecControls;

use mongodb::bson::doc;
use mongodb::options::ReplaceOptions;
use mongodb::results::{InsertOneResult, UpdateResult};
use mongodb::{Collection, Database};
use rocket::serde::Serialize;
use rocket::*;
use serde::DeserializeOwned;

use crate::dao::Dao;

pub struct ParsecDao {
    pub dao: Dao<ParsecControls>,
}

impl ParsecDao {
    pub fn new(database: Database) -> Self {
        Self {
            dao: Dao::new(database, "parsec_configs".into()),
        }
    }

    pub async fn get_parsec_controls(&self, id: &str) -> Option<ParsecControls> {
        let controls = self.dao.get_one_by_key("parsec_id", id).await;
        println!("GOT PARSEC CONTROLS {:?}", controls);
        controls
    }
    pub async fn insert_parsec_config(&self, config: ParsecControls) -> Option<UpdateResult> {
        println!("INSETTING PARSEC CONFIG");
        self.dao
            .insert_or_update(config.clone(), "parsec_id", config.parsec_id.as_str())
            .await
    }
}
