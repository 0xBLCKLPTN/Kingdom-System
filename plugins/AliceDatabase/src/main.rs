pub mod atypes;
pub mod configurator;
pub mod remore_db;

use atypes::*;
use configurator::*;
use std::env;
use remore_db::*;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file_path = &args[1];
    //let file_path = "/home/twofacedjanus/Documents/AliceDatabase/config_template.json";
    let config = configurator::read_config(file_path);
    match config {
        Ok(_) => println!("{:#?}", config.unwrap()),
        _ => panic!("Error while reading config file."),
    }

    //let remore_db_service = RemoreDBInstance::new("RemoreInstance".to_string(), None, None, "SuperKey.");
    //let key = "Users".to_string();
    //remore_db_service.set(key.clone(), CacheEntry {value: "['user1']".to_string()}).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Add;

    #[test]
    fn it_works() {
        let result = 2_i32.add(2);
        assert_eq!(result, 4);
    }
}