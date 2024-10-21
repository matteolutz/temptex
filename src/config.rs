use std::fs::File;

use serde_derive::{Deserialize, Serialize};

pub const CONFIG_FILE_PATH: &'static str = ".temptex.json";

#[derive(Serialize, Deserialize)]
pub struct Template {
    name: String,
    template_path: String,
    header_files: Vec<String>,
}

impl Template {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn template_path(&self) -> &str {
        &self.template_path
    }
    pub fn header_files(&self) -> &[String] {
        &self.header_files
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    templates: Vec<Template>,
    latex_command: String,
    aux_dir: String,
    pdf_dir: String,
}

impl Config {
    pub fn templates(&self) -> &[Template] {
        &self.templates
    }
    pub fn latex_command(&self) -> &str {
        &self.latex_command
    }
    pub fn aux_dir(&self) -> &str {
        &self.aux_dir
    }
    pub fn pdf_dir(&self) -> &str {
        &self.pdf_dir
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            templates: vec![],
            latex_command: "pdflatex".to_string(),
            aux_dir: "aux".to_string(),
            pdf_dir: "pdf".to_string(),
        }
    }
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    // let config_file_path = Path::new(CONFIG_FILE_PATH);
    let home_dir_buf = home::home_dir();
    if home_dir_buf.is_none() {
        return Err("Could not find home directory".into());
    }

    let config_file_path = home::home_dir().unwrap().as_path().join(CONFIG_FILE_PATH);

    if !config_file_path.exists() {
        println!(
            "[temptex] Config file not found, creating a new one at {}",
            config_file_path.display()
        );

        let default_config = Config::default();

        let f = File::create(config_file_path)?;
        serde_json::to_writer(f, &default_config)?;

        return Ok(default_config);
    }

    let file = File::open(config_file_path)?;
    let config: Config = serde_json::from_reader(file)?;

    Ok(config)
}
