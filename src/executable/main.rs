mod cli;
#[cfg(feature = "with-info")]
mod informer;
mod modifier;
mod util;

use clap::StructOpt;
use cli::CLIArgs;

use cli::Command;
#[cfg(feature = "with-info")]
use informer::execute_informer_op;
use modifier::execute_modifier_op;

fn main() {
    let args = CLIArgs::parse();

    match args.command {
        Command::Manager => eprintln!("Error: Manager is not enabled."),
        Command::Modifier { operation } => execute_modifier_op(operation),
        #[cfg(feature = "with-info")]
        Command::Info { operation } => execute_informer_op(operation),
    }
}
