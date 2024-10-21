use clap::{Parser, Subcommand};
use commands::{init::SubcommandInit, TemptexCommandRunner};

pub mod commands;
pub mod config;

#[derive(Subcommand)]
enum Subcommands {
    /// Init a new template in the current directory
    Init(SubcommandInit),
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct TemptexArgs {
    #[clap(subcommand)]
    command: Subcommands,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load_config()?;
    let args = TemptexArgs::parse();

    match args.command {
        Subcommands::Init(init_args) => init_args.run(config),
    }?;

    Ok(())
}
