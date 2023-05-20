use mongodb::{ Client, options::ClientOptions};

#[derive(Clone)]
pub struct MongoController {
    pub client_options: ClientOptions
}

impl MongoController {
    pub async fn new() -> Self {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
        client_options.app_name = Some("Kingdom-UserBase".to_string());
        
        Self { client_options }
    }
    pub async fn set_teacher(&self) {
        println!("Hello World!");
    } 
}
