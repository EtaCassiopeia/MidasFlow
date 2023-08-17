use std::process;

use clap::CommandFactory;
use clap::Parser;

use midas_cli::cli::helper::init_midas;
use midas_cli::cli::types::{Cli, LOGO};
use midas_cli::engine::SimpleOrchestrator;
use midas_cli::errors::{CliError, MidasError};
use midas_cli::{set_ctrl_handler, set_panic_hook, shutdown};
use midas_types::tracing::error;

fn main() {
    set_panic_hook();

    if let Err(e) = run() {
        error!("{}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), MidasError> {
    let cli = Cli::parse();
    let mut midas = init_orchestrator(&cli)?;
    let (shutdown_sender, shutdown_receiver) = shutdown::new(&midas.runtime);
    set_ctrl_handler(shutdown_sender);

    midas.run(shutdown_receiver)
}

fn init_orchestrator(cli: &Cli) -> Result<SimpleOrchestrator, CliError> {
    let res = init_midas(cli.config_paths.clone());

    match res {
        Ok(midas) => Ok(midas),
        Err(e) => {
            if let CliError::FailedToFindConfigurationFiles(_) = &e {
                let description = "Can not find configuration file";

                let mut command = Cli::command();
                command = command.about(format!("\n\n\n{} \n {}", LOGO, description));

                println!("{}", command.render_help());
            }

            error!("{}", e);
            Err(e)
        }
    }
}
