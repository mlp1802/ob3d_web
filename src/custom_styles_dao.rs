use core::custom_styles::serial_custom_style::SerialCustomStyle;
use core::parsec::parsec_controls::ParsecControls;

use mongodb::bson::doc;
use mongodb::options::ReplaceOptions;
use mongodb::results::{InsertOneResult, UpdateResult};
use mongodb::{Collection, Database};
use rocket::serde::Serialize;
use rocket::*;
use serde::DeserializeOwned;

use crate::dao::Dao;

pub struct CustomStylesDao {
    pub dao: Dao<SerialCustomStyle>,
}

impl CustomStylesDao {
    pub fn new(database: Database) -> Self {
        Self {
            dao: Dao::new(database, "custom_styles".into()),
        }
    }

    pub async fn get_custom_style(&self, id: &str) -> Option<SerialCustomStyle> {
        let custom_style = self.dao.get_one_by_key("id", id).await;
        println!("GOT  CUSTOM STYLE {:?}", custom_style);
        custom_style
    }
    pub async fn insert_custom_style(&self, config: SerialCustomStyle) -> Option<UpdateResult> {
        println!("INSETTING CUSTOM STYLE ");
        self.dao
            .insert_or_update(config.clone(), "id", config.id.as_str())
            .await
    }
}
