use mongodb::{
    Client,
    options::{
        ClientOptions, FindOptions,
    },
    Collection,
    error::Error as ER
};
use mongodb::bson::{self, oid::ObjectId, doc, to_document, Document};
use serde::{Deserialize, Serialize};
use crate::models::group_models::{NewGroup, Group};
use std::str::FromStr;
use futures::TryStreamExt;

#[derive(Clone, Debug)]
pub struct MongoGroupController {
    group_collection: Collection<Group>,
}


impl MongoGroupController {
    pub async fn new() -> Result<Self, ER> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        client_options.app_name = Some("KingdomBase".to_string());

        let client = Client::with_options(client_options)?;
        let database = client.database("Kingdom-Base");

        let mut group_collection = database.collection::<Group>("group_collection");
        
        println!("✅ Mongo Group Controller connected successfully");
        Ok(Self { group_collection })
    }

    pub async fn check_on_group_exist(&self, group_name: &str, course: &str) -> bool {
        let filter = doc! { "group_name": group_name, "course": course};
        let result = match self.group_collection.find_one(Some(filter), None).await.unwrap() {
            Some(group) => return true,
            None => return false,
        };

    }

    pub async fn new_group(&self, group: NewGroup) -> anyhow::Result<()>{
        if self.check_on_group_exist(&group.group_name, &group.course).await {
            return Ok(());
        }

        let ng = Group {
            id: ObjectId::new(),
            group_short_name: group.group_short_name.clone(),
            group_name: group.group_name.clone(),
            main_teacher_id: group.main_teacher_id,
            course: group.course.clone(),

        };
        self.group_collection.insert_one(ng, None).await.unwrap();
        return Ok(());

    }
    pub async fn delete_group(&self, group_id: &str) -> anyhow::Result<()> {
        let filter = doc! {"_id": ObjectId::parse_str(group_id).unwrap()};
        self.group_collection.delete_one(filter, None).await;
        Ok(())
    }

    pub async fn get_groups(&self) -> Vec<Group> {
        let mut cursor = self.group_collection.find(None, None).await.unwrap();
        let mut result: Vec<Group> = Vec::new();
        while let Some(group) = cursor.try_next().await.unwrap() {
            result.push(
                group
            ); 
        }
        result
    }
}