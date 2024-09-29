use std::path::PathBuf;
use crate::BoxedResult;
use crate::adbcore::fs_manager::*;
use std::io::ErrorKind;

lazy_static! {
    static ref ALICEDB_DEFAULT_PATH: PathBuf = PathBuf::from("./ALICEDB_DEFAULT_PATH");
}

pub async fn create_database(name: &str, engine: &str) -> Result<(), ErrorKind> {
    create_dir(&ALICEDB_DEFAULT_PATH.join(name)).await;
    return Ok(create_file(&ALICEDB_DEFAULT_PATH.join(name).join(".ADB_Init")).await?);
}

pub async fn get_databases() -> BoxedResult<Vec<PathBuf>> {
    return Ok(listdir(&ALICEDB_DEFAULT_PATH).await?);
}

pub async fn get_tables_in_database(db_name: &str) -> BoxedResult<Vec<PathBuf>> {
    return Ok(listdir(&ALICEDB_DEFAULT_PATH.join(db_name)).await?);
}

pub async fn get_tables_in_databases() -> BoxedResult<Vec<PathBuf>> {
    let res = get_databases().await?;
    let mut tables: Vec<PathBuf> = Vec::new();
    for i in res {
        if i.is_dir() {
            for k in listdir(&i).await? {
                tables.push(k);
            }
        }
        
    }
    Ok(tables)
}

pub async fn delete_database(db_name: &str) -> BoxedResult<()> {
    return Ok(delete_dir(&ALICEDB_DEFAULT_PATH.join(db_name)).await?);
}

pub async fn create_table(db_name: &str, table_name: &str) -> Result<(), ErrorKind> {
    return Ok(create_file(&ALICEDB_DEFAULT_PATH.join(db_name).join(table_name.to_owned() + ".alicedb")).await?);
}

pub async fn delete_table(db_name: &str, table_name: &str) -> BoxedResult<()> {
    return Ok(delete_file(&ALICEDB_DEFAULT_PATH.join(db_name).join(table_name.to_owned() + ".alicedb")).await?);
}

pub async fn write(db_name: &str, table_name: &str, data: &str) -> BoxedResult<()> {
    return Ok(simple_write(&ALICEDB_DEFAULT_PATH.join(db_name).join(table_name.to_owned() + ".alicedb"), data).await?)
}