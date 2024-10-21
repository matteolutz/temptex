use clap::{Parser, Subcommand};
use commands::{
    create::SubcommandCreate, init::SubcommandInit, list::SubcommandList, TemptexCommandRunner,
};

pub mod commands;
pub mod config;

#[derive(Subcommand)]
enum Subcommands {
    /// Instantiate a new template in the current directory
    Init(SubcommandInit),
    /// List all saved templates
    List(SubcommandList),
    /// Create a new template
    Create(SubcommandCreate),
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct TemptexArgs {
    #[clap(subcommand)]
    command: Subcommands,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = TemptexArgs::parse();
    let config = config::load_config()?;

    match args.command {
        Subcommands::Init(init_args) => init_args.run(config),
        Subcommands::List(list_args) => list_args.run(config),
        Subcommands::Create(create_args) => create_args.run(config),
    }?;

    Ok(())
}
