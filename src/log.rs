use std::path::Path;

const PATH: &str = "D:/Rust/files/";
pub struct Log<'a> {
    name: &'a str,
    date: chrono::NaiveDate,
    origin: &'a str,
    level: Level,
    msg: &'a str,
}

impl<'a> Log<'_> {
    pub fn log() {
        let name = get_input("Log name: ");
        let path = Path::new(PATH).join(name);
        println!("{:?}", path.to_str());
    }
}

struct Settings {
    file: String,
    level: Level,
    write: bool,
    terminal: bool,
    // color
}
impl Settings {
    fn settings() {}
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
fn get_input(q: &str) -> &str {
    println!("{q}");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    Box::leak(input.trim().to_string().into_boxed_str())
} //chat gpt lmao, no idea what Box does

// let date = chrono::Local::now().date_naive();
// this returns the system's local date.
// DateTime<Local> parsed to YYYY-MM-DD

// "files" path: D:\Rust\files

// Msg and Level -> input
