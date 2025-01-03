mod log;
mod origin2;
fn main() {
    let log1 = log::Log::setup("Logger 1");
    log1.info("This is a log info msg");
    log1.info("This is a log info msg2");
    // origin2::test();
}
