mod log;
fn main() {
    let log_with_json = log::Log::setup("Log with .json", "./Logs/", true);

    log_with_json.info("This is an Info msg.");
    log_with_json.debug("This is a Debug msg.");
    log_with_json.warning("This is a Warning msg.");
    log_with_json.error("This is an Error msg.");
}
