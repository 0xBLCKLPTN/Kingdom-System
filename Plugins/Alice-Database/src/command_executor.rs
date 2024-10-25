
/*                          MIT License

Copyright (c) 2024 Daniil Ermolaev

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE. */

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

