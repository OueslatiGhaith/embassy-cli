use cargo_generate::{GenerateArgs, TemplatePath};

pub struct GeneratorConfig {
    pub name: String,
    pub vendor: String,
    pub mcu: String,
    pub target: String,
}

pub fn create(cfg: GeneratorConfig) -> anyhow::Result<()> {
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

    let args = GenerateArgs {
        template_path,
        silent: true,
        verbose: true,
        name: Some(cfg.name),
        define: vec![
            format!("vendor={}", cfg.vendor),
            format!("mcu={}", cfg.mcu),
            format!("target={}", cfg.target),
        ],
        ..Default::default()
    };

    cargo_generate::generate(args)?;

    Ok(())
}
