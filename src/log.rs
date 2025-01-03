use chrono::Local;
use colored::{ColoredString, Colorize};
use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
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
    fn to_string(&self) -> String {
        let level = match self.level {
            Level::Info => String::from("Info"),
            Level::Debug => String::from("Debug"),
            Level::Warning => String::from("Warning"),
            Level::Error => String::from("Error"),
        };
        return format!(
            "Date: {}, level: {}, origin: {}, msg: {} \n",
            self.date, level, self.origin, self.msg
        );
    }
    fn print_content(self, settings: &Settings) {
        print!("Log: {} ", settings.name);
        print!("[{}] ", self.date);
        print!("{} ", self.origin);
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
                let mut txt_file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&txt_path)
                    .expect("OpenOptions error.");
                txt_file
                    .write_all(Content::to_string(&content).as_bytes())
                    .expect("Error writing content to .txt file.");
                if settings.terminal {
                    Content::print_content(content, &settings);
                    print!("{} ", "INFO".blue());
                    println!("Message: {}\n", color(settings.color, msg));
                }
            }
            Err(_) => println!("Error opening settings file."),
        }
    }
    pub fn debug(&self, msg: &str) {
        let settings_path = Path::new(PATH).join(self.name).join("settings.json");
        let txt_path = Path::new(PATH).join(self.name).join("logs.txt");
        let settings =
            fs::read_to_string(settings_path).expect("Error reading .json settings file.");
        match serde_json::from_str::<Settings>(&settings) {
            Ok(settings) => {
                let content = Content::content(Level::Debug, msg, "origin", settings.fmt);
                let mut txt_file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&txt_path)
                    .expect("OpenOptions error.");
                txt_file
                    .write_all(Content::to_string(&content).as_bytes())
                    .expect("Error writing content to .txt file.");
                if settings.terminal {
                    Content::print_content(content, &settings);
                    print!("{} ", "DEBUG".green());
                    println!("Message: {}\n", color(settings.color, msg));
                }
            }
            Err(_) => println!("Error opening settings file."),
        }
    }
    pub fn warning(&self, msg: &str) {
        let settings_path = Path::new(PATH).join(self.name).join("settings.json");
        let txt_path = Path::new(PATH).join(self.name).join("logs.txt");
        let settings =
            fs::read_to_string(settings_path).expect("Error reading .json settings file.");
        match serde_json::from_str::<Settings>(&settings) {
            Ok(settings) => {
                let content = Content::content(Level::Warning, msg, "origin", settings.fmt);
                let mut txt_file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&txt_path)
                    .expect("OpenOptions error.");
                txt_file
                    .write_all(Content::to_string(&content).as_bytes())
                    .expect("Error writing content to .txt file.");
                if settings.terminal {
                    Content::print_content(content, &settings);
                    print!("{} ", "WARNING".yellow());
                    println!("Message: {}\n", color(settings.color, msg));
                }
            }
            Err(_) => println!("Error opening settings file."),
        }
    }
    pub fn error(&self, msg: &str) {
        let settings_path = Path::new(PATH).join(self.name).join("settings.json");
        let txt_path = Path::new(PATH).join(self.name).join("logs.txt");
        let settings =
            fs::read_to_string(settings_path).expect("Error reading .json settings file.");
        match serde_json::from_str::<Settings>(&settings) {
            Ok(settings) => {
                let content = Content::content(Level::Error, msg, "origin", settings.fmt);
                let mut txt_file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&txt_path)
                    .expect("OpenOptions error.");
                txt_file
                    .write_all(Content::to_string(&content).as_bytes())
                    .expect("Error writing content to .txt file.");
                if settings.terminal {
                    Content::print_content(content, &settings);
                    print!("{} ", "ERROR".red());
                    println!("Message: {}\n", color(settings.color, msg));
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
            fmt: "%d-%m-%Y %H:%M:%S",
        };
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
    println!("Log created at path {}\n", path.to_str().unwrap());
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
    let datetime = Local::now();
    return datetime.format(fmt).to_string();
}
fn color<'a>(color: Option<&str>, colored: &str) -> ColoredString {
    match color {
        Some(color) => match color {
            "black" => return colored.black(),
            "red" => return colored.red(),
            "green" => return colored.green(),
            "yellow" => return colored.yellow(),
            "blue" => return colored.blue(),
            "magenta" => return colored.magenta(),
            "cyan" => return colored.cyan(),
            "white" => return colored.white(),
            _ => return colored.normal(),
        },
        None => return colored.normal(),
    }
}
