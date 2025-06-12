use std::env;

mod cli;

fn main() {
    env_logger::init();

    if let Err(e) = cli::run(env::args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
