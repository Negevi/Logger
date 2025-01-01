use colored::Color;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

const PATH: &str = "D:/Rust/logs/";

#[derive(Serialize, Deserialize, Debug)]
struct Content<'a> {
    date: String,
    origin: &'a str,
    level: Level,
    msg: &'a str,
}
impl<'a> Content<'a> {
    fn content() {}
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Log<'a> {
    name: &'a str,
    content: Option<Content<'a>>, // content mutavel, Option caso o log não tenh nada para logar. Achei meio esquisito, mas foi a solução q achei
}

impl<'a> Log<'_> {
    pub fn setup(name: &str) -> Result<Log, Error> {
        let path = Path::new(PATH).join(name);
        println!("path {:?}", path.to_str()); // debug
        match fs::create_dir(&path) {
            Ok(_) => {
                create_files(path, name); // creates .txt, .json and folder for each logger
                let log = Log {
                    name: name,
                    content: None,
                };
                Ok(log)
            }
            Err(_) => {
                println!("Error creating new log. Please check for repetitive log names.");
                Err(Error::InvalidLog)
            }
        }
    }
    pub fn info(&self, msg: &str) {
        let settings_path = Path::new(PATH).join(self.name).join("settings.json");
        let settings = fs::read_to_string(settings_path);
        match settings {
            Ok(settings) => {
                println!("{}", settings)
            }
            Err(_) => println!("Error opening settings file."),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    name: String,
    path: PathBuf,
    terminal: bool,
    color: Option<String>,
}
impl Settings {
    pub fn settings(path: PathBuf, name: &str) -> Settings {
        let config = Settings {
            name: name.to_string(),
            path: path,
            terminal: false,
            color: None,
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
#[derive(Deserialize, Serialize, Debug)]
pub enum Error {
    InvalidType,
    InvalidLog,
}

fn create_files(path: PathBuf, name: &str) {
    println!("Log created at path {:?}", path.to_str());
    let settings = Settings::settings(path, name);
    let json_string = serde_json::to_string_pretty(&settings).unwrap();
    let json_path = Path::new(&settings.path).join("settings.json");
    let mut file = File::create(json_path).expect("Error creating .json file");
    file.write_all(json_string.as_bytes())
        .expect("Error writing string to json.");
    let txt_path = Path::new(&settings.path).join("logs.txt");
    File::create(txt_path).expect("Error creating .txt file");
}

fn local_date_string() -> String {
    let date = chrono::Local::now().date_naive();
    return date.to_string();
}

// let date = chrono::Local::now().date_naive();
// this returns the system's local date.
// DateTime<Local> parsed to YYYY-MM-DD
