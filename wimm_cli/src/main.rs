use std::env;

use wimm_core::App;

mod cli;

fn main() {
    env_logger::init();

    if let Err(e) = cli::get_args(env::args()).and_then(|args| {
        App::new(args.db_path)?.run(&args.action)?;
        Ok(())
    }) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
