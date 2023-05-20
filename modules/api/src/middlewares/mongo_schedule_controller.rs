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
use crate::models::schedule_models::*;
use std::str::FromStr;
use futures::TryStreamExt;


#[derive(Clone, Debug)]
pub struct MongoScheduleController {
    schedule_collection: Collection<Schedule>,
}

impl MongoScheduleController {
    pub async fn new() -> Result<Self, ER> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        client_options.app_name = Some("Kingdom-Base".to_string());

        let client = Client::with_options(client_options)?;
        let database = client.database("Kingdom-Base");

        let mut schedule_collection = database.collection::<Schedule>("schedule_collection");
        
        println!("✅ Mongo Schedule Controller connected successfully");
        Ok(Self { schedule_collection })
    }


    pub async fn add_schedule(&self, new_lesson: NewSchedule) -> anyhow::Result<()> {

        let nl = Schedule {
            id: ObjectId::new(),
            date: new_lesson.date,
            group_id: new_lesson.group_id,
            lessons: new_lesson.lessons,

        };
        self.schedule_collection.insert_one(nl, None).await.unwrap();
        return Ok(());
    }

    pub async fn delete_schedule(&self, schedule_id: &str) -> anyhow::Result<()> {
        let filter = doc! {"_id": ObjectId::parse_str(schedule_id).unwrap()};
        self.schedule_collection.delete_one(filter, None).await;
        Ok(())
    }

    pub async fn get_schedules(&self) -> Vec<Schedule> {
        let mut cursor = self.schedule_collection.find(None, None).await.unwrap();
        let mut result: Vec<Schedule> = Vec::new();
        while let Some(schedule) = cursor.try_next().await.unwrap() {
            result.push(
                schedule
            ); 
        }
        result
    }
}