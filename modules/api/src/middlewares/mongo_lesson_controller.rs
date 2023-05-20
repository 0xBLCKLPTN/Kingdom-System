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
use crate::models::lesson_models::{Lesson, NewLesson};
use std::str::FromStr;
use futures::TryStreamExt;


#[derive(Clone, Debug)]
pub struct MongoLessonController {
    lesson_collection: Collection<Lesson>,
}



impl MongoLessonController {
    pub async fn new() -> Result<Self, ER> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        client_options.app_name = Some("Kingdom-Base".to_string());

        let client = Client::with_options(client_options)?;
        let database = client.database("Kingdom-Base");

        let mut lesson_collection = database.collection::<Lesson>("lesson_collection");
        
        println!("✅ Mongo Lesson Controller connected successfully");
        Ok(Self { lesson_collection })
    }

    pub async fn check_on_lesson_exist(&self, lesson_name: &str) -> bool{
        let filter = doc! { "lesson_name": lesson_name};
        let result = match self.lesson_collection.find_one(Some(filter), None).await.unwrap() {
            Some(lesson) => return true,
            None => return false,
        };
    }

    pub async fn add_lesson(&self, new_lesson: NewLesson) -> anyhow::Result<()> {
        if self.check_on_lesson_exist(&new_lesson.lesson_name).await {
            return Ok(());
        }

        let nl = Lesson {
            id: ObjectId::new(),
            teacher_id: new_lesson.teacher_id.clone(),
            lesson_name: new_lesson.lesson_name.clone(),

        };
        self.lesson_collection.insert_one(nl, None).await.unwrap();
        return Ok(());
    }

    pub async fn delete_lesson(&self, lesson_id: &str) -> anyhow::Result<()> {
        let filter = doc! {"_id": ObjectId::parse_str(lesson_id).unwrap()};
        self.lesson_collection.delete_one(filter, None).await;
        Ok(())
    }

    pub async fn get_lessons(&self) -> Vec<Lesson> {
        let mut cursor = self.lesson_collection.find(None, None).await.unwrap();
        let mut result: Vec<Lesson> = Vec::new();
        while let Some(lesson) = cursor.try_next().await.unwrap() {
            result.push(
                lesson
            ); 
        }
        result
    }
}