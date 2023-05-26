use serde::{ Deserialize, Serialize };
use serde_json::Result;
use std::fs;

use structopt::StructOpt;


#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
pub struct Opt {
    #[structopt(short = "d", long = "debug", help = "Activate debug mode.")]
    debug: bool,
    #[structopt(short = "c", long = "config", help = "Configuration file.")]
    pub conf: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub host_url: String,
}

pub fn configure(conf: &str) -> Result<Config>{
    let filepath = conf.to_owned();
    let contents = fs::read_to_string(filepath).expect("Couldn't find or load file.");
    
    let config: Config = serde_json::from_str(&contents)?;
    Ok(config)
}
