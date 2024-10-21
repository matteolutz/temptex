use crate::config::Config;

pub mod create;
pub mod init;
pub mod list;

pub trait TemptexCommandRunner {
    fn run(&self, config: Config) -> Result<(), Box<dyn std::error::Error>>;
}
