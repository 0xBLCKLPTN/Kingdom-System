use crate::fs_manager::*;
use std::path::PathBuf;

pub fn create_log_table() -> bool
{
    if !list_dir("./LOGDATABASE").unwrap().contains(&PathBuf::from(r"./LOGDATABASE/loggings.alicedb.log")){
        create_dir("./LOGDATABASE".to_string()).unwrap();
        create_file("./LOGDATABASE/loggings.alicedb.log".to_string()).unwrap();
        return false;
    }
    return true;
}