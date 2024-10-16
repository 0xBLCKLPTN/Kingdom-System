// Engines
#[derive(Debug)]
pub struct JSONEngine {
    pub name: String,
}

use pest::Parser;

#[derive(Parser, Debug)]
#[grammar = "grammar.pest"] // Путь к вашему файлу грамматики
pub struct WhiteRabbit;

impl JSONEngine{
    pub fn new(wre: WhiteRabbit) -> Self {
        Self { name: "qweqwe".to_string(), white_rabbit_engine: wre}
    }
}



pub async fn white_rabbit(cmd: &str) {
    let pairs = WhiteRabbit::parse(Rule::ident_list, cmd).expect("I cant parse this command.");
    for pair in pairs {
        match pair.as_rule() {
            Rule::get_all_instances => {
                println!("Getting all instances.");
            },
            Rule::select_instance => {
                println!("Instance selected! {:#?}", pair.into_inner().as_str());
            },
            Rule::print_instance => {
                println!("ADBInstance: {:#?}", pair.into_inner().as_str());
            },
            Rule::delete_instance => {
                println!("Instance deleted: {:#?}", pair.into_inner().as_str());
            },
            Rule::check_instance_on_exists => {
                println!("Instance exists? {:#?}", pair.into_inner().as_str());
            },
            Rule::create_collection => {
                println!("Collection created! {:#?}", pair.into_inner().as_str());
            },
            Rule::remove_collection => {
                println!("Remove collection! {:#?}", pair.into_inner().as_str());
            },
            Rule::add_document_to_collection => {
                println!("Add document to collection {:#?}", pair.into_inner());
            },
            Rule::remove_document_from_collection => {
                println!("Remove document from collection {:#?}", pair.into_inner());
            },
            _ => unreachable!(),
        }
    }
}
