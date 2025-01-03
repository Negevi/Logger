mod log;
mod origin2;
fn main() {
    let log = log::Log::setup("Math");
    log.info("This is an info msg.");
    log.debug("This is an debug msg.");
    log.warning("This is an warning msg.");
    log.error("This is an error msg.");

    log::Log::setup("log2");
}
