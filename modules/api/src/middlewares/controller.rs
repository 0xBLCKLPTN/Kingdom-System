use mongodb::{ Client, Collection, Database };
use mongodb::options::{ ClientOptions };
use mongodb::error::Error as MongoError;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct MongoController<T> {
    pub collection: Collection<T>,
}

async fn get_database() -> Result<Database, MongoError> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("Kingdom-Base".to_string());

    let client = Client::with_options(client_options)?;
    let database = client.database("Kingdom-Base");
    
    Ok(database)
}


impl<T> MongoController<T>
where T: Debug + Clone + Send + Sync,
{
    pub async fn new(collection_name: &str) -> Result<MongoController<T>, MongoError> {
        let database = get_database().await?;
        let collection: Collection<T> = database.collection::<T>(collection_name);
        println!("✅ Mongo {} Controller connected successfully", collection_name);
        Ok( MongoController { collection } )
    }

}
