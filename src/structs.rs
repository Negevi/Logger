pub struct Log {
    name: String,
    date: chrono::NaiveDate,
    origin: String,
    level: Level,
    msg: String,
}

impl Log {
    pub fn create(name: String, level: Level, msg: String) -> Log {
        let date = chrono::Local::now().date_naive();
        let origin: String = "origin".to_string();
        let log: Log = Log {
            name,
            date,
            origin,
            level,
            msg,
        };
        return log;
    }
}
pub enum Level {
    Info,
    Debug,
    Warning,
    Error,
}

enum Error {
    InvalidType,
}

pub fn get_input(q: &str) -> String {
    let mut input = String::new();
    println!("{q}");
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(_) => get_input(q),
    }
}

pub fn get_input_i32(q: &str) -> i32 {
    let mut input = String::new();
    println!("{q}");
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().parse::<i32>().unwrap(),
        Err(_) => get_input_i32(q),
    }
}

// let date = chrono::Local::now().date_naive();
// this returns the system's local date.
// DateTime<Local> parsed to YYYY-MM-DD

// "files" path: D:\Rust\files
