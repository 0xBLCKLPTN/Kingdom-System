use std::io::{self, Write};
use crate::instance::*;

pub fn cli(instance_manager: &mut InstanceManager) {
    loop {
        print!("[ Instance Manager ] (type 'exit' to quit)=: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed_input = input.trim();

                if trimmed_input.eq_ignore_ascii_case("exit") {
                    println!("Exiting...");
                    break;
                }

                instance_manager.wrapped_execute_cmd(trimmed_input);
            }
            Err(error) => {
                println!("Error reading input: {}", error);
            }
        }
    }
}
