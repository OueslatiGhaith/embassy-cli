use std::{io::Write, path::PathBuf};

use crate::git::Git;

use super::generate::GeneratorConfig;

pub struct TemplateBuilder {
    root: Template,
    cfg: GeneratorConfig,
}

impl TemplateBuilder {
    pub async fn new(cfg: GeneratorConfig) -> anyhow::Result<Self> {
        Template::Dir {
            name: "dir_1".into(),
            children: vec![
                Template::File {
                    name: "file_1".into(),
                    content: "hello world".into(),
                },
                Template::Dir {
                    name: "dir_2".into(),
                    children: vec![],
                },
            ],
        };

        Ok(Self {
            root: Template::root(&cfg).await?,
            cfg,
        })
    }

    pub fn build(&self) -> anyhow::Result<PathBuf> {
        for item in self.root.flatten(&".".into()) {
            match item {
                TemplateItem::Dir { path } => std::fs::DirBuilder::new().create(path)?,
                TemplateItem::File { content, path } => {
                    let mut f = std::fs::File::create(path)?;
                    f.write_all(content.as_bytes())?;
                }
            }
        }

        Ok(self.cfg.name.clone().into())
    }
}

enum Template {
    File {
        name: String,
        content: String,
    },
    Dir {
        name: String,
        children: Vec<Template>,
    },
}

impl Template {
    async fn root(cfg: &GeneratorConfig) -> anyhow::Result<Self> {
        Ok(Template::Dir {
            name: cfg.name.clone(),
            children: Vec::from([
                Template::dot_cargo(cfg).await?,
                Template::app_src(cfg).await?,
                Template::dot_gitignore(cfg).await?,
                Template::build_rs(cfg).await?,
                Template::root_cargo_toml(cfg).await?,
                Template::rust_toolchain(cfg).await?,
            ]),
        })
    }

    async fn dot_cargo(cfg: &GeneratorConfig) -> anyhow::Result<Self> {
        let mcu = cfg.mcu.as_str();
        let target = cfg.target.as_str();

        Ok(Template::Dir {
            name: ".cargo".into(),
            children: vec![Template::File {
                name: "config.toml".into(),
                content: format!(
                    r#"[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = "probe-run --chip {mcu} --speed 1000 --connect-under-reset"
                
[build]
target = "{target}"
                
[env]
DEFMT_LOG = "trace""#
                ),
            }],
        })
    }

    async fn app_src(cfg: &GeneratorConfig) -> anyhow::Result<Self> {
        let embassy_crate = vendor_to_crate(&cfg.vendor);

        Ok(Template::Dir {
            name: "src".into(),
            children: vec![Template::File {
                name: "main.rs".into(),
                content: format!(
                    r#"#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::{{Duration, Timer}};
use {{defmt_rtt as _, panic_probe as _}};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {{
    let p = {embassy_crate}::init(Default::default());

    info!("Hello World!");

    loop {{
        Timer::after(Duration::from_millis(500)).await;
        info!("Hello!");
    }}
}}"#
                ),
            }],
        })
    }

    async fn dot_gitignore(_cfg: &GeneratorConfig) -> anyhow::Result<Self> {
        Ok(Template::File {
            name: ".gitignore".into(),
            content: "/target".into(),
        })
    }

    async fn build_rs(_cfg: &GeneratorConfig) -> anyhow::Result<Self> {
        Ok(Template::File {
            name: "build.rs".into(),
            content: r#"fn main() {
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}"#
            .into(),
        })
    }

    async fn root_cargo_toml(cfg: &GeneratorConfig) -> anyhow::Result<Self> {
        let name = cfg.name.as_str();
        let embassy_crate = vendor_to_crate(&cfg.vendor);
        let crate_decl = crate_declaration(&cfg.vendor, &cfg.mcu).await?;
        let commit = if cfg.no_pin {
            "".into()
        } else {
            let hash = Git::get_latest_commit().await?;
            format!(r#"commit = "{hash}""#)
        };
        let version_executor = Git::get_crate_version("embassy-executor").await?;
        let version_time = Git::get_crate_version("embassy-time").await?;
        let version_sync = Git::get_crate_version("embassy-sync").await?;
        let version_futures = Git::get_crate_version("embassy-futures").await?;

        Ok(Template::File {
            name: "Cargo.toml".into(),
            content: format!(
                r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
{crate_decl}
embassy-executor = {{ version = "{version_executor}", features = ["nightly", "arch-cortex-m", "executor-thread", "integrated-timers"] }}
embassy-time = {{ version = "{version_time}", features = ["defmt", "defmt-timestamp-uptime", "tick-hz-32_768"] }}
embassy-sync = {{ version = "{version_sync}", features = ["defmt"] }}
embassy-futures = {{ version = "{version_futures}" }}

panic-probe = {{ version = "0.3" }}

defmt = {{ version = "0.3" }}
defmt-rtt = {{ version = "0.4" }}

cortex-m = {{ version = "0.7.6", features = ["critical-section-single-core"] }}
cortex-m-rt = "0.7.0"

futures = {{ version = "0.3.17", default-features = false, features = ["async-await"] }}

[patch.crates-io]
{embassy_crate} = {{ git = "https://github.com/embassy-rs/embassy", {commit} }}
embassy-executor = {{ git = "https://github.com/embassy-rs/embassy", {commit} }}
embassy-time = {{ git = "https://github.com/embassy-rs/embassy", {commit} }}
embassy-sync = {{ git = "https://github.com/embassy-rs/embassy", {commit} }}
embassy-futures = {{ git = "https://github.com/embassy-rs/embassy", {commit} }}"#
            ),
        })
    }

    async fn rust_toolchain(cfg: &GeneratorConfig) -> anyhow::Result<Self> {
        let channel = Git::get_toolchain_channel().await?;
        let target = cfg.target.as_str();

        Ok(Template::File {
            name: "rust-toolchain.toml".into(),
            content: format!(
                r#"[toolchain]
channel = "{channel}"
components = [ "rust-src", "rustfmt", "llvm-tools" ]
targets = [ "{target}" ]"#
            ),
        })
    }

    fn flatten(&self, root_path: &PathBuf) -> Vec<TemplateItem> {
        fn traverse(node: &Template, path: &PathBuf, stack: &mut Vec<TemplateItem>) {
            match node {
                Template::File { name, content } => stack.push(TemplateItem::File {
                    content: content.clone(),
                    path: path.join(name),
                }),
                Template::Dir { name, children } => {
                    stack.push(TemplateItem::Dir {
                        path: path.join(name),
                    });

                    for child in children {
                        traverse(child, &path.join(name), stack)
                    }
                }
            }
        }

        let mut stack = vec![];
        traverse(self, root_path, &mut stack);
        stack
    }
}

enum TemplateItem {
    File { content: String, path: PathBuf },
    Dir { path: PathBuf },
}

fn vendor_to_crate(vendor: &str) -> &str {
    match vendor.to_lowercase().as_str() {
        "st" => "embassy-stm32",
        "nrf" => "embassy-nrf",
        "rp" => "embassy-rp",
        _ => unreachable!(),
    }
}

async fn crate_declaration(vendor: &str, mcu: &str) -> anyhow::Result<String> {
    let embassy_crate = vendor_to_crate(vendor);
    let version = Git::get_crate_version(embassy_crate).await?;

    let r = match vendor.to_lowercase().as_str() {
        "st" => format!(
            r#"embassy-stm32 = {{ version = "{version}", features = ["nightly", "defmt", "time-driver-any", "{mcu}", "memory-x", "exti"] }}"#
        ),
        "nrf" => format!(
            r#"embassy-nrf = {{ version = "{version}", features = ["nightly", "defmt", "{mcu}", "time-driver-rtc1", "gpiote"] }}"#
        ),
        "rp" => format!(
            r#"embassy-rp = {{ version = "{version}", features = ["defmt", "nightly", "time-driver"] }}"#
        ),
        _ => unreachable!(),
    };

    Ok(r)
}
