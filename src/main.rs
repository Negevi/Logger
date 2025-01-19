mod log;
fn main() {
    let log1 = log::Log::setup("Log 1", "D:/Rust/logs/");
    log1.info("This is an info msg.");
    log1.warning("This in an error msg.");
    log1.debug("This is a debug msg.");
    log1.error("This is a error msg.");
}
