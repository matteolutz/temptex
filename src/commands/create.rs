use clap::Args;

use super::TemptexCommandRunner;

#[derive(Args)]
pub struct SubcommandCreate {
    /// Path to the template file
    #[clap(value_parser)]
    file: String,

    /// The name of the template. If not given, this will fallback to the name of the template file
    #[clap(short, long, value_parser)]
    template_name: Option<String>,

    /// The header files to be included when instantiating the template
    #[clap(short = 'i', long, value_parser, num_args = 1..)]
    header_files: Vec<String>,
}

impl TemptexCommandRunner for SubcommandCreate {
    fn run(&self, config: crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
