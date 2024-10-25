use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "syntax/instance_manager.pest"]
struct InstanceManagerIdentParser;

pub trait IMCommandExecutor {
    fn execute(&mut self, command: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Attempt to parse the command and handle any parsing errors
        match InstanceManagerIdentParser::parse(Rule::sql_statements, command) {
            Ok(pairs) => {
                // Process parsed pairs
                for pair in pairs {
                    for inner_pair in pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::create_instance => {
                                println!("Creating new instance...");
                            },
                            Rule::get_instance => {
                                println!("Retrieving instance...");
                            },
                            Rule::get_instances => {
                                println!("Retrieving all instances...");
                            },
                            Rule::delete_instance => {
                                println!("Deleting instance...");
                            },
                            _ => println!("Unrecognized command."),
                        }
                    }
                }
                Ok(())
            }
            // Handle parsing errors without panicking
            Err(e) => {
                println!("Error parsing command: {}", e);
                Err(Box::new(e)) // Return the error wrapped in a Box
            }
        }
    }
}

