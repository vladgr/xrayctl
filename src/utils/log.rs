use log::LevelFilter;
use std::io::Write;

pub fn init_logger() {
    env_logger::Builder::new()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .filter(None, LevelFilter::Info)
        .init();
}
