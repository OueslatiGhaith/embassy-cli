use std::io::Write;

use cargo_generate::{GenerateArgs, TemplatePath};

use crate::git::Git;

use super::data::EmbassyCrates;

pub struct GeneratorConfig {
    pub name: String,
    pub vendor: String,
    pub mcu: String,
    pub target: String,
    pub no_pin: bool,
}

pub async fn create(cfg: GeneratorConfig) -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    let template_path = TemplatePath {
        path: Some("./template".to_owned()),
        ..Default::default()
    };

    #[cfg(not(debug_assertions))]
    let template_path = TemplatePath {
        git: Some("https://github.com/OueslatiGhaith/embassy-cli".to_owned()),
        subfolder: Some("./template".to_owned()),
        ..Default::default()
    };

    let latest_commit = if !cfg.no_pin {
        Some(Git::get_latest_commit().await?)
    } else {
        None
    };

    let toolchain_channel = Git::get_toolchain_channel().await?;

    let mut definitions = vec![
        format!("vendor={}", cfg.vendor),
        format!("mcu={}", cfg.mcu),
        format!("target={}", cfg.target),
        format!("toolchain_channel={}", toolchain_channel),
    ];

    if let Some(commit) = latest_commit {
        definitions.push(format!("commit={}", commit))
    }

    let mut default_crates = EmbassyCrates::default_crates();

    match cfg.vendor.to_lowercase().as_str() {
        "st" => default_crates.push(EmbassyCrates::Stm32),
        "nrf" => default_crates.push(EmbassyCrates::Nrf),
        "rp" => default_crates.push(EmbassyCrates::Rp),
        _ => unreachable!(),
    }

    for embassy_crate in default_crates {
        let name: String = embassy_crate.into();
        let version = Git::get_crate_version(&name).await?;
        definitions.push(format!("versions_{}={}", name, version));
    }

    let args = GenerateArgs {
        template_path,
        silent: true,
        verbose: true,
        name: Some(cfg.name),
        define: definitions,
        ..Default::default()
    };

    let path = cargo_generate::generate(args)?;

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
