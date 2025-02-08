mod log;
fn main() {
    let log1 = log::Log::setup("Log 1", "./Logs/", false);
    log1.info("This is an info msg.");
    log1.debug("This is a debug msg.");
    log1.warning("This is a warning msg.");
    log1.error("This is an error msg.");
}
