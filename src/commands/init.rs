use clap::Args;

use super::TemptexCommandRunner;

#[derive(Args)]
pub struct SubcommandInit {
    /// The name of the template
    #[clap(value_parser)]
    name: String,
}

impl TemptexCommandRunner for SubcommandInit {
    fn run(&self, config: crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        println!("running this...");
        Ok(())
    }
}
