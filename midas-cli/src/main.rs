use std::process;

use midas_cli::errors::MidasError;
use midas_cli::set_panic_hook;
use midas_types::tracing::error;

fn main() {
    set_panic_hook();

    if let Err(e) = run() {
        error!("{}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), MidasError> {
    Ok(())
}
