use crate::fs_manager::*;

pub fn create_log_table()
{
    create_dir("./LOGDATABASE".to_string()).unwrap();
    create_file("./LOGDATABASE/logging.alicedb.log".to_string()).unwrap();
}