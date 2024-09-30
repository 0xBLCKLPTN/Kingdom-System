use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    /* 
    if cfg!(target_os = "windows") {
        println!("NONE");
    } else {
        Command::new("mkdir ~/ALICEDB_DATA").status().expect("Failed to create dir.");
        Command::new("export ALICEDATABSE_DEFAULT_PATH=$HOME/ALICEDB_DATA").status().expect("failed to add dir");
    }
    
    //  Define the path to the folder you want to create
    let dirs = vec![
        Path::new("$HOME/alicedb_path/ALICEDB_DATA"),
        //Path::new("target/dir2"), // Windows
        //Path::new("target/dir3/subdir"),  // *
    ];

    // Create the directories if they don't exist
    for dir in dirs {
        if !dir.exists() {
            fs::create_dir_all(dir).expect("Failed to create directory");
            println!("Created directory: {:?}", dir);
        } else {
            println!("Directory already exists: {:?}", dir);
        }
    }
    */
    println!("BUILDING ALICEDB..");
}