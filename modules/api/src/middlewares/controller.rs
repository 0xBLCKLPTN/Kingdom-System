use mongodb::{ Client, Collection, Database };
use mongodb::options::{ ClientOptions, FindOptions };
use mongodb::bson::{ self, doc, to_document, Document };
use mongodb::bson::oid::ObjectId;
use mongodb::error:Error as MongoError;

use serde::{ Deserialize, Serialize };
use std::str::FromStr;
use futures::TryStreamExt;
use std::fmt::Debug;

use crate::models::database_models::*;


#[derive(Clone, Debug)]
pub struct MongoController<T> {
    collection: Collection<T>,
}

pub async fn get_database() -> Result<Database, MongoError> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("Kingdom-Base".to_string());

    let client = Client::with_options(client_options)?;
    let database = client.database("Kingdom-Base");
    
    Ok(database)
}

impl<T> MongoController<T>
where T: Debug + Clone,
{
    pub async fn new(collection_name: &str) -> Result<MongoController<T>, MongoError> {
        let database = get_database().await?;
        let collection = database.collection::<T>(collection_name);
        println!("✅ Mongo {} Controller connected successfully", collection_name);
        Ok( MongoController { collection } )
    }

}


