mod log;
fn main() {
    let log1 = log::Log::setup("Log 1");
    log1.info("This is an info msg.");
}
