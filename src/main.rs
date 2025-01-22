mod log;
fn main() {
    let log1 = log::Log::setup("Log 1", "D:/Rust/logs/");
    log1.info("This is an info msg.");
}
