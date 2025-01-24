use chrono::Local;
use serde::{Deserialize, Serialize};
use std::backtrace::Backtrace;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
struct Content<'a> {
    date: String,
    origin: String,
    level: Level,
    msg: &'a str,
}
impl<'a> Content<'a> {
    fn content(level: Level, msg: &'a str, origin: String, fmt: &str) -> Content<'a> {
        let content = Content {
            date: local_date_string(fmt),
            level: level,
            origin: origin,
            msg: msg,
        };
        return content;
    }
    fn to_string(&self) -> String {
        format!(
            "Date: {}, Level: {:?}, Origin: {}, Message: {}\n",
            self.date, self.level, self.origin, self.msg
        )
    }
    fn print_log(self, settings: &Settings, level: &str, msg: &str) {
        let level_color = match level {
            "INFO" => "\x1b[36m",
            "DEBUG" => "\x1b[32m",
            "WARNING" => "\x1b[33m",
            "ERROR" => "\x1b[31m",
            _ => "\x1b[0m",
        };
        match settings.color {
            None => print!(
                "{}: [{}]  {} {level_color} [ {} ] \x1b[0m {}\n",
                settings.name, self.date, self.origin, level, msg
            ),
            Some(mut color) => {
                match color {
                    "red" => color = "\x1b[31m",
                    "green" => color = "\x1b[32m",
                    "yellow" => color = "\x1b[33m",
                    "blue" => color = "\x1b[34m",
                    "magenta" => color = "\x1b[35m",
                    "cyan" => color = "\x1b[36m",
                    _ => {
                        color = "\x1b[37m";
                        println!("Color variable is invalid. Please check for any typing mishaps in the .json file.\nAvailable colors: red, green, yellow, blue, magenta, cyan\nFor none, please use the keyword null.\n")
                    }
                }
                print!(
                    "{color} {}: [{}] {} {level_color} [ {} ] \x1b[0m {color} {} \x1b[0m \n",
                    settings.name, self.date, self.origin, level, msg
                );
            }
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Log<'a> {
    name: &'a str,
    path: &'a str,
    content: Option<Content<'a>>, // content mutavel, Option caso o log não tenha nada para logar. Achei meio esquisito, mas foi a solução q achei
}
impl<'a> Log<'_> {
    pub fn setup(name: &'a str, path: &'a str) -> Log<'a> {
        let pathbuf = Path::new(path).join(name);
        match fs::create_dir(&pathbuf) {
            Ok(_) => {
                create_files(pathbuf, name); // creates .txt, .json and folder for each logger (clone fn for debug, remove later)
                println!("Log files created at {}", path); // Debug
                let log = Log {
                    name: name,
                    path: path,
                    content: None,
                };
                return log;
            }
            Err(_) => {
                let log = Log {
                    name: name,
                    path: path,
                    content: None,
                };
                return log;
            }
        }
    }
    fn log(&self, msg: &str, level: Level) {
        let str_level = level.as_str();
        let settings_path = Path::new(self.path).join(self.name).join("settings.json");
        let txt_path = Path::new(self.path).join(self.name).join("logs.txt");
        match serde_json::from_str::<Settings>(
            &fs::read_to_string(settings_path).unwrap_or_default(),
        ) {
            Ok(settings) => {
                let bt = Backtrace::force_capture().to_string();
                let content = Content::content(
                    level,
                    msg,
                    parse_bt(bt).unwrap_or_default(),
                    settings.datefmt,
                );
                let mut txt_file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&txt_path)
                    .expect("Error opening the file's OpenOptions");
                txt_file
                    .write_all(Content::to_string(&content).as_bytes())
                    .expect("Error writing content to .txt file.");
                if settings.terminal {
                    Content::print_log(content, &settings, str_level, msg);
                }
            }
            Err(_) => println!("Error opening settings file."),
        }
    }

    pub fn info(&self, msg: &str) {
        self.log(msg, Level::Info);
    }

    pub fn debug(&self, msg: &str) {
        self.log(msg, Level::Debug);
    }

    pub fn warning(&self, msg: &str) {
        self.log(msg, Level::Warning);
    }

    pub fn error(&self, msg: &str) {
        self.log(msg, Level::Error);
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Settings<'a> {
    name: String,
    path: PathBuf,
    terminal: bool,
    color: Option<&'a str>,
    datefmt: &'a str,
}
impl<'a> Settings<'a> {
    pub fn settings(path: PathBuf, name: &str) -> Settings {
        let config = Settings {
            name: String::from(name),
            path: path,
            terminal: true,
            color: None,
            datefmt: "%d-%m-%Y %H:%M:%S",
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
impl Level {
    fn as_str(&self) -> &'static str {
        return match self {
            Level::Info => "INFO",
            Level::Debug => "DEBUG",
            Level::Warning => "WARNING",
            Level::Error => "ERROR",
        };
    }
}
#[derive(Deserialize, Serialize, Debug)]
pub enum Error {
    InvalidType,
    InvalidLog,
    InvalidOrigin,
    InvalidFile,
}
fn create_files(path: PathBuf, name: &str) {
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
fn parse_bt(bt: String) -> Option<String> {
    let mut lines = bt.lines();
    while let Some(line) = lines.next() {
        if line.contains("log.rs") {
            let _blank = lines.next();
            let origin: &str = lines.next().unwrap().trim();
            let polished_origin = String::from(origin).split_off(4);
            return Some(String::from(polished_origin));
        }
    }
    None
}
// No error handling, dont know if it even works ;...(
