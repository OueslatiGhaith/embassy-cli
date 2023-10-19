use std::io::Write;

use crate::commands::create::Vendor;

use super::templates::TemplateBuilder;

pub struct GeneratorConfig {
    pub name: String,
    pub vendor: Vendor,
    pub mcu: String,
    pub target: String,
    pub no_pin: bool,
    pub workspace: bool,
}

pub async fn create(cfg: GeneratorConfig) -> anyhow::Result<()> {
    let path = TemplateBuilder::new(cfg).await?.build()?;

    // run cargo fmt
    std::process::Command::new("cargo")
        .arg("fmt")
        .current_dir(&path)
        .output()?;

    // format toml files
    let toml_paths = [path.join("Cargo.toml"), path.join("./.cargo/config.toml")];
    for toml_path in toml_paths {
        let toml = std::fs::read_to_string(&toml_path)?;
        let mut toml = toml.parse::<toml_edit::Document>()?;
        toml.as_table_mut().fmt();

        let as_string = toml.to_string();
        let mut file = std::fs::File::create(toml_path)?;
        let as_string = remove_triple_newlines(&as_string);
        file.write_all(as_string.as_bytes())?;
    }

    Ok(())
}

fn remove_triple_newlines(string: &str) -> String {
    let mut new_string = String::new();
    for char in string.chars() {
        if char == '\n' && new_string.ends_with("\n\n") {
            continue;
        }
        new_string.push(char);
    }
    new_string
}
