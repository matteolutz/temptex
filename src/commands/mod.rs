use crate::config::Config;

pub mod init;

pub trait TemptexCommandRunner {
    fn run(&self, config: Config) -> Result<(), Box<dyn std::error::Error>>;
}
