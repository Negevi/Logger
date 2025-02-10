mod log;
fn main() {
    let log_with_json = log::Log::setup("Log with .json", "pathhere", true);

    log_with_json.info("This is an Info msg.");
    log_with_json.debug("This is a Debug msg.");
    log_with_json.warning("This is a Warning msg.");
    log_with_json.error("This is an Error msg.");

    let log_without_json = log::Log::setup("Log without .json", "pathhere", false).color("cyan").datefmt("%Y-%m-%d %H:%M:%S");

    log_without_json.info("This is an Info msg.");
    log_without_json.debug("This is a Debug msg.");
    log_without_json.warning("This is a Warning msg.");
    log_without_json.error("This is an Error msg.");

    log::quick_log(log::Level::Info, "Global Log Info msg.");
    log::quick_log(log::Level::Debug, "Global Log Debug msg.");
    log::quick_log(log::Level::Warning, "Global Log Warning msg.");
    log::quick_log(log::Level::Error, "Global Log Error msg.");
}
