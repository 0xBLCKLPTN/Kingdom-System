//!
//!    Filename: fs_manager.rs
//!    Description: Module that wrap all fs functions.
//!    Author: Daniil (0xJanus) Ermolaev
//!

use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::io::ErrorKind;
use std::ffi::OsString;

use std::io::{ self, prelude::*, BufReader, SeekFrom };

use std::path::{PathBuf, Path};

use crate::BoxedResult;


/// Returns true if file exists or false if not.
///
/// # Examples
///
/// ```
/// if check_on_file_exists(&PathBuf::from("main.rs")).await { todo!() }
/// ```
pub async fn check_on_file_exists(filepath: &PathBuf) -> bool {
    match filepath.try_exists() {
        Ok(_) => return true,
        _ => return false,
    }
}

/// Returns string from &PathBuf
///
/// # Examples
///
/// ```
/// let pathbuf = PathBuf::from("main.rs");
/// let k = pathbuf_to_string(&pathbuf).await.unwrap();
/// ```
pub async fn pathbuf_to_string(pathbuf: &PathBuf) -> Result<String, OsString>{
    return pathbuf.clone().into_os_string().into_string()
}

/// Returns vector of files like Vec<PathBuf>.
///
/// # Examples
///
/// ```
/// let files = listdir(&PathBuf::from("./Docs")).await.unwrap();
/// ```
pub async fn listdir(dirpath: &PathBuf) -> BoxedResult<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = vec![];
    match fs::read_dir(pathbuf_to_string(&dirpath).await.unwrap()) {
        Err(why) => println!( "!{:?}", why.kind() ),
        Ok(paths) => for path in paths {
            files.push(path.unwrap().path());

        },
    };
    Ok(files)
}

/// Creates a dir in folderpath.
///
/// # Examples
///
/// ```
/// let _ = create_dir(PathBuf::from("./newdir")).await.unwrap();
/// ```
pub async fn create_dir(dirpath: &PathBuf) -> BoxedResult<()> {
    match fs::create_dir(&pathbuf_to_string(dirpath).await.unwrap()) {
        Err(why) => println!("CD !{:?}", why.kind()),
        Ok(_) => println!("Done!"),
    };
    Ok(())
}

/// Deletes a dir using folderpath.
///
/// # Examples
///
/// ```
/// let _ = delete_dir(PathBuf::from("./newdir")).await.unwrap();
/// ```
pub async fn delete_dir(dirpath: &PathBuf) -> BoxedResult<()> {
    match fs::remove_dir(&pathbuf_to_string(dirpath).await.unwrap()) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => println!("Done!"),
    };
    Ok(())
}

/// Creates file using filepath.
///
/// Examples
///
/// ```
/// let _ = create_file(&PathBuf::from("./testfile")).await.unwrap();
/// ```
pub async fn create_file(filepath: &PathBuf) -> Result<(), ErrorKind> {
    return match File::create(pathbuf_to_string(filepath).await.unwrap()) {
        Err(why) => Err(why.kind()),
        Ok(_) => Ok(()),
    };
}

/// Deletes file using filepath.
///
/// Examples
///
/// ```
/// let _ = delete_file(&PathBuf::from("./testfile")).await.unwrap();
/// ```
pub async fn delete_file(filepath: &PathBuf) -> BoxedResult<()> {
    match fs::remove_file(pathbuf_to_string(filepath).await.unwrap()) {
        Err(why) => println!("!{:?}", why.kind()),
        Ok(_) => println!("Done!"),
    }
    Ok(())
}

/// Write some data to file.
///
/// Examples
///
/// ```
/// let _ = write(&PathBuf::from("./testfile", "[LOG]->DATA")).await.unwrap()
/// ```
pub async fn simple_write(filepath: &PathBuf, data: &str) -> BoxedResult<()> {
    let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&pathbuf_to_string(filepath).await.unwrap())
                    .unwrap();
    if let Err(why) = writeln!(file, "{}", data.to_string()) {
        eprintln!("Couldn't write to file: {}", why);
    }
    Ok(())
}

/// Read some data from file.
/// 
/// Examples
/// 
/// ```
/// let data = read_file(&PathBuf::from("./testfile")).await.unwrap();
/// ```
pub async fn read_file(filepath: &PathBuf) -> BoxedResult<String> {
    let mut out = fs::read_to_string(filepath).unwrap();
    return Ok(out);
}