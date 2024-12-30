use serde::{Deserialize, Serialize};
use std::fs::{self, DirBuilder, File};
use std::io::Write;
use std::path::{Path, PathBuf};

const PATH: &str = "D:/Rust/logs/";
pub struct Log<'a> {
    name: &'a str,
    date: chrono::NaiveDate,
    origin: &'a str,
    level: Level,
    msg: &'a str,
}

impl<'a> Log<'_> {
    pub fn new_log(name: &str) {
        let path = Path::new(PATH).join(name);
        println!("path {:?}", path.to_str()); // debug
        match fs::create_dir(path.clone()) {
            Ok(_) => {
                println!("Log created at path {:?}", path.to_str());
                let settings = Settings::settings(path);
                let json_string = serde_json::to_string_pretty(&settings).unwrap();
                let json_path = Path::new(&settings.path).join("settings");
                let mut file = File::create(json_path).expect("Error creating file");
                file.write_all(json_string.as_bytes())
                    .expect("Error writing string to json.");
            }
            Err(_) => println!("That Log already exists. Please try gain"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    path: PathBuf,
    level: Level,
    terminal: bool,
    // color
}
impl Settings {
    pub fn settings(path: PathBuf) -> Settings {
        let config = Settings {
            path: path,
            level: Level::Info,
            terminal: false,
        };
        println!("{:?}", config); // debug
        return config;
    }
}
#[derive(Deserialize, Serialize, Debug)]

pub enum Level {
    Info,
    Debug,
    Warning,
    Error,
}
enum Error {
    InvalidType,
}

// let date = chrono::Local::now().date_naive();
// this returns the system's local date.
// DateTime<Local> parsed to YYYY-MM-DD
