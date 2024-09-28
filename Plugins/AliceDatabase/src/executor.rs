use std::collections::HashMap;

pub struct CommandHandler {
    commands: HashMap<String, Box<dyn Fn(&str) -> Result<(), String>>>,
}

impl CommandHandler {
    pub fn new() -> Self {
        let mut commands = HashMap::new();
        commands.insert("CREATE TABLE".to_string(), Box::new(|args: &str| create_table(args)) as Box<dyn Fn(&str) -> Result<(), String>>);
        commands.insert("DROP TABLE".to_string(), Box::new(|args: &str| drop_table(args)) as Box<dyn Fn(&str) -> Result<(), String>>);
        commands.insert("CREATE DATABASE".to_string(), Box::new(|args: &str| drop_table(args)) as Box<dyn Fn(&str) -> Result<(), String>>);
        Self { commands }
    }

    pub fn handle_command(&self, command: &str) -> Result<(), String> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Err("Empty command".to_string());
        }

        let command_name = parts[0].to_uppercase() + " " + parts[1].to_uppercase().as_str();
        let command_args = parts[2..].join(" ");

        if let Some(func) = self.commands.get(&command_name) {
            func(&command_args)
        } else {
            Err(format!("Unknown command: {}", command_name))
        }
    }
}

pub fn create_table(args: &str) -> Result<(), String> {
    let table_name = args.trim();
    if table_name.is_empty() {
        return Err("Table name is required".to_string());
    }

    // Simulate creating a table
    println!("Creating table: {}", table_name);
    Ok(())
}

pub fn drop_table(args: &str) -> Result<(), String> {
    let table_name = args.trim();
    if table_name.is_empty() {
        return Err("Table name is required".to_string());
    }

    // Simulate dropping a table
    println!("Dropping table: {}", table_name);
    Ok(())
}