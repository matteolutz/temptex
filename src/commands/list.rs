use clap::Args;

use crate::config::Config;

use super::TemptexCommandRunner;

#[derive(Args)]
pub struct SubcommandList;

impl TemptexCommandRunner for SubcommandList {
    fn run(&self, config: Config) -> Result<(), Box<dyn std::error::Error>> {
        println!("[temptex] Templates:");
        let templates = config.templates();
        for template in templates {
            println!(" - {}", template.name());
        }
        Ok(())
    }
}
