use chrono::Local;
use serde::{Deserialize, Serialize};
use std::backtrace::Backtrace;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
struct Content<'a> { // the quick_log() fn uses the Content struct as the logger
    date: String,
    origin: String,
    level: Level,
    msg: &'a str,
}
impl<'a> Content<'a> {
    fn new(level: Level, msg: &'a str, origin: String, fmt: &str) -> Content<'a> {
        let content: Content<'a> = Content {
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
    fn print_log(self, settings: &Settings, msg: &str) {
        let level_color = match self.level.as_str() {
            "INFO" => "\x1b[36m",
            "DEBUG" => "\x1b[32m",
            "WARNING" => "\x1b[33m",
            "ERROR" => "\x1b[31m",
            _ => "\x1b[0m",
        };
        match settings.color {
            None => print!(
                "{}: [{}]  {} {level_color} [ {} ] \x1b[0m {}\n",
                settings.name, self.date, self.origin, self.level.as_str(), msg
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
                    "{color}{}: [{}] {} {level_color} [ {} ] \x1b[0m {color} {} \x1b[0m \n",
                    settings.name, self.date, self.origin, self.level.as_str(), msg
                );
            }
        }
    }
    fn quick_log_content(level: Level, msg: &'a str, bt: String) {
        let content = Content::new(level, msg, parse_bt(bt).unwrap_or_default(), "%d-%m-%Y %H:%M:%S");
        Content::print_log(content, &Settings::quick_settings(), msg)
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Log<'a> {
    name: &'a str,
    path: &'a str,
    settings: Option<Settings<'a>>, //option settings, caso tenha ou nao o .json
    content: Option<Content<'a>>, // content mutavel, Option caso o log não tenha nada para logar. Achei meio esquisito, mas foi a solução q achei
}
impl<'a> Log<'a> {
    fn new(name: &'a str, path: &'a str, settings: Option<Settings<'a>>) -> Self {
        let log: Log<'_> = Log {
            name: name,
            path: path,
            settings: settings,
            content: None,
        };
        return log;
    }
    pub fn setup(name: &'a str, path: &'a str, json: bool) -> Log<'a> {
        let pathbuf = Path::new(path).join(name);
        if json {
            match fs::create_dir(&pathbuf) {
                Ok(_) => {
                    let log = Log::new(name, path, None);
                    create_files_json(pathbuf, name);
                    return log;
                }
                Err(_) => {
                    let log = Log::new(name, path, None);
                    return log;
                }
            }
        } else {
            match fs::create_dir(&pathbuf) {
                Ok(_) => {
                    let log = Log::new(name, path, Some(Settings::new(name, pathbuf.clone(), json, true, None, "%d-%m-%Y %H:%M:%S")));
                    create_files(&pathbuf);
                    return log;
                }
                Err(_) => {
                    let log = Log::new(name, path, Some(Settings::new(name, pathbuf.clone(), json, true, None, "%d-%m-%Y %H:%M:%S")));
                    return log;
                }
            }
        }
    }

    fn log(&self, msg: &'a str, level: Level, origin: String) {
        let txt_path = Path::new(self.path).join(self.name).join("logs.txt");
        if self.settings.is_none() { // with .json settings
            let settings_path = Path::new(self.path).join(self.name).join("settings.json");
            match serde_json::from_str::<Settings>(
                &fs::read_to_string(settings_path).unwrap_or_default(),
            ) {
                Ok(settings) => {
                    let content: Content<'a> = Content::new(
                        level,
                        msg,
                        parse_bt(origin).unwrap_or_default(),
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
                        Content::print_log(content, &settings, msg);
                    }
                }
                Err(_) => println!("Error opening settings file. Make sure to not use equal logger names and to have a valid path"),
            }
        } else {  //without .json settings 
            match &self.settings  {
                Some(settings) => {
                    let content: Content<'a> = Content::new(
                        level,
                        msg,
                        parse_bt(origin).unwrap_or_default(),
                        settings.datefmt,
                    );
                    let mut txt_file = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(&txt_path)
                        .expect("Error opening the file's OpenOptions. Please check to your path fmt (.../example/example/) ");
                    txt_file
                        .write_all(Content::to_string(&content).as_bytes())
                        .expect("Error writing content to .txt file.");
                    if settings.terminal {
                        Content::print_log(content, &settings, msg);
                    }
                }
                None => println!("Fetching Settings variable"),
                }
            }
        }

    pub fn info(&self, msg: &str) {
        let bt = Backtrace::force_capture().to_string();
        self.log(msg, Level::Info, bt);
    }

    pub fn debug(&self, msg: &str) {
        let bt = Backtrace::force_capture().to_string();
        self.log(msg, Level::Debug, bt);
    }

    pub fn warning(&self, msg: &str) {
        let bt = Backtrace::force_capture().to_string();
        self.log(msg, Level::Warning, bt);
    }

    pub fn error(&self, msg: &str) {
        let bt = Backtrace::force_capture().to_string();
        self.log(msg, Level::Error, bt);
    }

    pub fn terminal(self, terminal: bool) -> Log<'a> {
        if self.settings.clone().is_some() && self.settings.clone().unwrap().terminal != terminal {
            let new_settings = Settings::new(self.name, PathBuf::from(self.path), false, terminal, self.settings.clone().unwrap().color, self.settings.unwrap().datefmt);
            let new_log = Log::new(self.name, self.path, Some(new_settings));
            return new_log;
        }
        else {
            println!("Methods are meant to be used with non .json setting loggers. If you wish to change some configuration, please refer to the .json configuration file. ");
            return self;
        }
    }

    pub fn color(self, color: &'a str) -> Log<'a> {
        if self.settings.clone().is_some() && self.settings.clone().unwrap().color != Some(color) {
            let new_settings = Settings::new(self.name, PathBuf::from(self.path), false, self.settings.clone().unwrap().terminal, Some(color), self.settings.unwrap().datefmt);
            let new_log = Log::new(self.name, self.path, Some(new_settings));
            return new_log;
        }
        else {
            println!("Methods are meant to be used with non .json setting loggers. If you wish to change some configuration, please refer to the .json configuration file. ");
            return self;
        }
    }

    pub fn datefmt(self, datefmt: &'a str) -> Log<'a> {
        if self.settings.clone().is_some() && self.settings.clone().unwrap().datefmt != datefmt {
            let new_settings = Settings::new(self.name, PathBuf::from(self.path), false, self.settings.clone().unwrap().terminal, self.settings.unwrap().color, datefmt);
            let new_log = Log::new(self.name, self.path, Some(new_settings));
            return new_log;
        }
        else {
            println!("Methods are meant to be used with non .json setting loggers. If you wish to change some configuration, please refer to the .json configuration file. ");
            return self;
        }
    }

    
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings<'a> {
    name: String,
    path: PathBuf,
    terminal: bool,
    json: bool,
    color: Option<&'a str>,
    datefmt: &'a str,
}
impl<'a> Settings<'a> {
    fn new(name: &str, path: PathBuf, json: bool, terminal: bool, color: Option<&'a str>, datefmt: &'a str) -> Settings<'a> {
        let config: Settings<'_> = Settings {
            name: String::from(name),
            path: path,
            terminal: terminal,
            json: json,
            color: color,
            datefmt: datefmt,
        };
        return config;
    }
    fn quick_settings() -> Settings<'a> {
        return Settings::new("Global Log", PathBuf::from("blank"), false, true, None, "%d-%m-%Y %H:%M:%S");
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
fn create_files_json(path: PathBuf, name: &str) {
    let settings = Settings::new(name, path, true, true, None, "%d-%m-%Y %H:%M:%S");
    let json_string = serde_json::to_string_pretty(&settings).unwrap();
    let json_path = Path::new(&settings.path).join("settings.json");
    let mut file = File::create(json_path).expect("Error creating .json file");
    file.write_all(json_string.as_bytes())
        .expect("Error writing string to json.");
    let txt_path = Path::new(&settings.path).join("logs.txt");
    File::create(txt_path).expect("Error creating .txt file");
}
fn create_files(path: &PathBuf) {
    let txt_path = Path::new(path).join("logs.txt");
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
pub fn quick_log(level: Level, msg: &str) {
    let bt = Backtrace::force_capture().to_string();
    Content::quick_log_content(level, msg, bt);
}