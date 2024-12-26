pub struct Log<'a> {
    name: &'a str,
    date: chrono::NaiveDate,
    origin: &'a str,
    level: Level,
    msg: &'a str,
}

impl<'a> Log<'_> {
    pub fn create_log() {}
}

struct Settings {
    file: String,
    level: Level,
    write: bool,
    terminal: bool,
}
impl Settings {
    fn settings(name: &str) {
        if name.ends_with(name) {}
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

// let date = chrono::Local::now().date_naive();
// this returns the system's local date.
// DateTime<Local> parsed to YYYY-MM-DD

// "files" path: D:\Rust\files

// Msg and Level -> input
