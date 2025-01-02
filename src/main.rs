mod log;
fn main() {
    let log = log::Log::setup("log1");
    log.info("This is an info msg.");
}
