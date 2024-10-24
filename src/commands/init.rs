use crate::config::{Config, Template};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use clap::Args;
use std::env;

use super::TemptexCommandRunner;

#[derive(Args)]
pub struct SubcommandInit {
    /// The name of the template
    #[clap(value_parser)]
    name: String,
}

impl SubcommandInit {
    fn copy_template_file(
        template: &Template,
        file_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create_new(file_path)?;

        let mut template_file = File::open(template.template_path())?;
        let mut template_file_buffer = String::new();
        template_file.read_to_string(&mut template_file_buffer)?;

        for head in template.header_files() {
            writeln!(file, "\\input{{{}}}", head)?;
        }

        writeln!(file)?;
        writeln!(file, "{}", template_file_buffer)?;

        Ok(())
    }

    fn generate_makefile(
        template: &Template,
        config: &Config,
        file_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create_new(file_path)?;

        writeln!(file, "LATEX = {}", config.latex_command())?;
        writeln!(file, "AUX_DIR = {}", config.aux_dir())?;
        writeln!(file, "PDF_DIR = {}", config.pdf_dir())?;
        writeln!(
            file,
            "LATEX_ARGS = --shell-escape -aux-directory $(AUX_DIR) -output-directory $(PDF_DIR)"
        )?;
        writeln!(file, "SRC = {}", template.name())?;

        writeln!(file, "$(SRC).pdf: $(SRC).tex")?;
        writeln!(file, "\t$(LATEX) $(LATEX_ARGS) $(SRC)")?;
        writeln!(file, "\t$(LATEX) $(LATEX_ARGS) $(SRC)")?;

        writeln!(file, "watch: $(SRC).tex")?;
        writeln!(
            file,
            "\tlatexmk -pdf -pvc -aux-directory=$(AUX_DIR) -output-directory=$(PDF_DIR) $(SRC)"
        )?;

        Ok(())
    }
}

impl TemptexCommandRunner for SubcommandInit {
    fn run(&self, config: Config) -> Result<(), Box<dyn std::error::Error>> {
        println!("[temptex] Look for template \"{}\"...", self.name);

        let template_opt = config.templates().iter().find(|t| t.name() == self.name);
        if template_opt.is_none() {
            return Err("Template not found".into());
        }
        let template = template_opt.unwrap();

        let cwd = env::current_dir()?;
        let main_tex_file = cwd.join(format!("{}.tex", template.name()));
        let make_file = cwd.join("Makefile");

        println!("[temptex] Found it! Copying template file...");
        Self::copy_template_file(template, main_tex_file.as_path())?;

        println!("[temptex] Generating Makefile...");
        Self::generate_makefile(template, &config, make_file.as_path())?;

        println!("[temptex] Done. Compile the file using \"make\"!");

        Ok(())
    }
}
