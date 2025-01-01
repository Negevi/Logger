use log::Log;

mod log;
fn main() {
    let log = log::Log::setup("log1").unwrap();
    log.info("bro");
}
