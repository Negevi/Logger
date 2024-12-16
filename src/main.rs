mod structs;
fn main() {
    let input = structs::get_input_i32(
        "Welcome to the logger. What would you like to do?:
    [0] Open settings
    [1] Create a Log
    [2] Exit code",
    );
    match input {
        0 => println!("opening settings..."),
        1 => println!("creating log..."),
        2 => std::process::exit(1),
        _ => println!("Ops, thats invalid!"),
    }
    let log = structs::Log::create(
        structs::get_input("Name?"),
        structs::Level::Info,
        "This is a msg.".to_string(),
    );
}
