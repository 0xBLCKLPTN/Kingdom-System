use crate::BoxedResult;

use pest_derive::Parser;
use pest::Parser;


#[derive(Parser)]
#[grammar = "sql.pest"]
struct IdentParser;


pub async fn execute_command(command: &str) -> BoxedResult<()> {
    let pairs = IdentParser::parse(Rule::sql_statements, command).expect("unsuccessful parse");

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::create_table => println!("Creating table: {:?}", inner_pair.into_inner().as_str()),
                Rule::create_database => println!("Creating database: {:?}", inner_pair.into_inner().as_str()),
                _ => unreachable!("I dont know this command."),
            }
            
        }
        
    }
    Ok(())
}