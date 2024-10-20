use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use serde_json::{json, Value};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct JSONEngine {
	pub name: String,
	pub data: HashMap<String, Value>, // Изменили на HashMap
}

impl JSONEngine {
	pub fn new(name: String) -> Self {
		Self { name, data: HashMap::new() }
	}

	pub fn insert_one(&mut self, new_data: Value) -> String {
		let name = Uuid::new_v4().to_string();
		self.data.insert(name.clone(), new_data);
		name
	}

	pub fn create_from_exist(instace_name: String, name: String) -> std::io::Result<Vec<String>> {
		let filepath = format!("ADB_Data/json_engine/{}", name);
		let mut files = Vec::new();
		for entry in std::fs::read_dir(filepath)? {
			let entry = entry?;
			let path = entry.path();
			if path.is_file() {
				if let Some(file_name) = path.file_name() {
					if let Some(file_name_str) = file_name.to_str() {
						files.push(format!("ADB_Data/json_engine/{}/{}",instance_name,file_name_str.to_string());
					}
				}
			}
		}
		Ok(files)
	}

	pub fn print_all(&self, key: Option<String>) {
		match key {
			Some(k) => {
				if let Some(value) = self.data.get(&k) { // Проверяем наличие ключа в HashMap
					println!("{:#?}", value);
				} else {
					println!("Ключ '{}' не найден", k);
				}
			},
			None => println!("{:#?}", self.data),
		}
	}

	pub fn commit(&self) -> Value {
		let key = json!(self.data.clone());
		// Запись данных в файл
		let file_path = "data.json";
		let json_string = serde_json::to_string_pretty(&self.data).unwrap();


		let mut file = File::create(file_path).expect("Не удалось создать файл");
		file.write_all(json_string.as_bytes()).expect("Не удалось записать в файл");

		key
	}
}

#[tokio::main]
async fn main() {
	let mut je = JSONEngine::create_from_exist("instance_1".to_string());
	println!("JE {:#?}",je );
}
