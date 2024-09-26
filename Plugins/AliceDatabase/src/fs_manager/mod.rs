use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::fs;
use std::path::{PathBuf, Path};
use std::fs::OpenOptions;
use std::io::{self, prelude::*, BufReader, SeekFrom};

pub fn list_dir(dirname: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files: Vec<PathBuf> = Vec::new();
    match fs::read_dir(dirname) {
       Err(why) => println!("! {:?}", why.kind()),
       Ok(paths) => for path in paths {
            files.push(path.unwrap().path());
       },
    };
    Ok(files)
}

pub fn create_dir(dirname: String) -> Result<(), Box<dyn Error>> {
    match fs::create_dir(&dirname) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => println!("Done!"), 
    };
    Ok(())
}


pub fn create_file(filename: String) -> Result<(), Box<dyn Error>> {
    let file = File::create(&filename)?;
    Ok(())
}

pub fn write_into_file(filename: String, data: String) {
    let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&filename)
                    .unwrap();
    if let Err(e) = writeln!(file, "{}", data) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

pub fn into_field(path_to_table: String, field_name: &str, data: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(&path_to_table)?;
    let mut reader = BufReader::new(&file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let mut lines: Vec<_> = contents.lines().map(|l| l.to_string()).collect();
    for line in &mut lines {
        if line.contains(field_name) {
            let k = format!("{},{}", line.replace("\n", ""), data);
            *line = k;
        }
    }

    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    file.write_all(lines.join("\n").as_bytes())?;

    Ok(())
}