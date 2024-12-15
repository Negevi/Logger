pub struct Log {
    name: String,
    date: chrono::Local,
    origin: String,
    level: Level,
    msg: String,
}

impl Log {
    fn create(name: String, date: chrono::Local, origin: String, level: Level, msg: String) -> Log {
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

enum Level {
    Info,
    Debug,
    Warning,
    Error,
}

enum Error {
    InvalidType,
}

fn get_input(q: &str) -> String {
    let mut input = String::new();
    println!("{q}");
    std::io::stdin().read_line(&mut input);
    return input;
}

// let date = chrono::Local::now().date_naive();
// this returns the system's local date.
// DateTime<Local> parsed to YYYY-MM-DD
