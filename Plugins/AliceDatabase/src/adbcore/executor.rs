use crate::BoxedResult;
use std::path::PathBuf;
use crate::engines::default_adbp::*;
use pest_derive::Parser;
use pest::Parser;

use std::io::ErrorKind;

use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Parser)]
#[grammar = "sql.pest"]
struct IdentParser;

pub async fn execute_command(command: &str) -> BoxedResult<()> {
    let pairs = IdentParser::parse(Rule::sql_statements, command).expect("unsuccessful parse");
    //println!("{:#?}", pairs);
    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::create_table => {
                    let val =inner_pair.into_inner().as_str().split(".").collect::<Vec<_>>();
                    println!("Creating table: {:?} in database: {:?}", val[1], val[0]);
                    match create_table(val[1], val[0]).await {
                        ErrorKind => {
                            create_database(val[1], "default").await;
                            create_table(val[1], val[0]).await;
                        }
                    }
                    
                },
                Rule::create_database => {
                    let val = inner_pair.into_inner().as_str().split_whitespace().collect::<Vec<_>>();
                    println!("Creating database: {:?} engine {:?}", val[0], val[2]);
                    println!("{:#?}", create_database(val[0], val[2]).await);
                },
                Rule::select_from => {
                    let val = inner_pair.into_inner().as_str().split_whitespace().collect::<Vec<_>>();
                    println!("Select {:#?} from {:#?}" , val[0], val[2]);
                },
                Rule::insert_data => {
                    //println!("{:#?}", inner_pair.into_inner());
                    let val = inner_pair.into_inner().as_str().split(" INTO ").collect::<Vec<_>>();
                    let k = val[1].split(".").collect::<Vec<_>>();
                    println!("{:#?}", k);
                    write(k[0], k[1], val[0].replace(")", "").as_str()).await;
                    println!("Insert {:#?} INTO {:#?}", val[0].replace(")", "").as_str(), val[1]);
                },
                Rule::create_user => {
                    let val = inner_pair.into_inner().as_str().split_whitespace().collect::<Vec<_>>();
                    println!("Create user {:#?} with password {:#?}" , val[0], val[2]);
                }
                Rule::delete_user => {
                    println!("Delete user {:#?}" , inner_pair.into_inner().as_str());
                },
                Rule::delete_database => {
                    println!("Delete database {:#?}", inner_pair.into_inner().as_str());
                },
                Rule::delete_table => {
                    println!("Delete table {:#?}", inner_pair.into_inner().as_str());
                },
                Rule::print_alldb => {
                    println!("INFO: {:#?}", get_tables_in_databases().await);
                }

                //println!("Select {:#?} from {:#?}", inner_pair.clone().into_inner().as_str(), inner_pair.into_inner()[1])
                _ => unreachable!("I dont know this command."),
            }
            
        }
        
    }
    Ok(())
}