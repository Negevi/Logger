use colored::{ColoredString, Colorize};
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
    fn content(level: Level, msg: &'a str, origin: &'a str, fmt: &str) -> Content<'a> {
        let content = Content {
            date: local_date_string(fmt),
            level: level,
            origin: origin,
            msg: msg,
        };
        return content;
    }
    fn print_content(self, settings: &Settings) {
        print!("{} ", settings.name);
        print!("{} ", self.date);
        print!("{}", self.origin);
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Log<'a> {
    name: &'a str,
    content: Option<Content<'a>>, // content mutavel, Option caso o log não tenha nada para logar. Achei meio esquisito, mas foi a solução q achei
}

impl<'a> Log<'_> {
    pub fn setup(name: &str) -> Log {
        let path = Path::new(PATH).join(name);
        println!("path {:?}", path.to_str()); // debug
        match fs::create_dir(&path) {
            Ok(_) => {
                create_files(path, name); // creates .txt, .json and folder for each logger
                let log = Log {
                    name: name,
                    content: None,
                };
                return log;
            }
            Err(_) => {
                let log = Log {
                    name: name,
                    content: None,
                };
                return log;
            }
        }
    }
    pub fn info(&self, msg: &str) {
        let settings_path = Path::new(PATH).join(self.name).join("settings.json");
        let txt_path = Path::new(PATH).join(self.name).join("logs.txt");
        let settings =
            fs::read_to_string(settings_path).expect("Error reading .json settings file.");
        match serde_json::from_str::<Settings>(&settings) {
            Ok(settings) => {
                let content = Content::content(Level::Info, msg, "origin", settings.fmt);
                fs::write(
                    &txt_path,
                    serde_json::to_string(&content).expect("Error writing content to string."),
                )
                .expect("Error writing log to .txt file.");
                if settings.terminal {
                    Content::print_content(content, &settings);
                    print!("{}", "INFO".blue());
                    println!("{}", color(settings.color, msg));
                }
            }
            Err(_) => println!("Error opening settings file."),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings<'a> {
    name: String,
    path: PathBuf,
    terminal: bool,
    color: Option<&'a str>,
    fmt: &'a str,
}
impl<'a> Settings<'a> {
    pub fn settings(path: PathBuf, name: &str) -> Settings {
        let config = Settings {
            name: String::from(name),
            path: path,
            terminal: true,
            color: None,
            fmt: "%Y-%m-%d %H:%M:%S",
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

fn local_date_string(fmt: &str) -> String {
    let date = chrono::Local::now().date_naive();
    let formatted = format!("{}", date.format(fmt)); // dont know if this works
    return formatted;
}

fn color<'a>(color: Option<&str>, colored: &str) -> ColoredString {
    match color.unwrap() {
        "black" => return colored.black(),
        "red" => return colored.red(),
        "green" => return colored.green(),
        "yellow" => return colored.yellow(),
        "blue" => return colored.blue(),
        "magenta" => return colored.magenta(),
        "cyan" => return colored.cyan(),
        "white" => return colored.white(),
        _ => return colored.normal(),
    } // Weird func.
}
