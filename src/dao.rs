use core::parsec::parsec_controls::ParsecControls;

use mongodb::bson::doc;
use mongodb::options::ReplaceOptions;
use mongodb::results::{InsertOneResult, UpdateResult};
use mongodb::{Collection, Database};
use rocket::serde::Serialize;
use rocket::*;
use serde::DeserializeOwned;

///general DAO
pub struct Dao<T>
where
    T: Serialize,
{
    _t: Option<T>,
    collection: Collection<T>,
}

impl<T> Dao<T>
where
    T: Serialize + DeserializeOwned + Unpin + Send + Sync,
{
    pub async fn insert_one(&self, object: T) -> Option<InsertOneResult>
    where
        T: serde::Serialize,
    {
        // let db: mongodb::Database = self.client.database("my_database");
        let insert_result = self
            .collection
            .insert_one(object, None)
            .await
            .map(|x| Some(x))
            .unwrap_or(None);
        insert_result
        //insert_result
    }
    /// Example "get_one" by _key (expand as needed)
    pub async fn get_one_by_key(&self, key_name: &str, value: &str) -> Option<T> {
        let filter = doc! { key_name: value };
        println!("Filter {:?}", filter);
        let res: Result<Option<T>, mongodb::error::Error> =
            self.collection.find_one(filter, None).await;
        match res {
            Ok(result) => {
                println!("Got result ");
                result
            }
            Err(e) => {
                eprintln!("Failed get_one_by_key: {}", e);
                None
            }
        }
    }

    pub async fn insert_or_update(
        &self,
        object: T,
        key_name: &str,
        key_value: &str,
    ) -> Option<UpdateResult> {
        // Build a filter like: { key_name: key_value }
        let filter = doc! { key_name: key_value };

        // ----- Approach A: "Full Replacement" of the existing doc -----
        // If you want to store the entire object `object`
        // and replace any existing doc with that entire object:
        let replace_options = ReplaceOptions::builder().upsert(true).build();
        // "replace_one" effectively removes the old doc and replaces with `object`.
        // Upsert means: if filter not matched, insert it.
        let replace_result = self
            .collection
            .replace_one(filter, object, replace_options)
            .await;

        match replace_result {
            Ok(update_result) => Some(update_result),
            Err(e) => {
                eprintln!("Failed upsert: {}", e);
                None
            }
        }
    }
    pub fn new(db: Database, collection_name: String) -> Self {
        Self {
            _t: None,
            collection: db.collection(&collection_name),
        }
    }
}

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
