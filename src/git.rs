use anyhow::anyhow;
use toml::Table;

pub struct Git;

impl Git {
    pub async fn get_latest_commit() -> anyhow::Result<String> {
        let raw_commit = reqwest::get("https://github.com/embassy-rs/embassy/commit/main.patch")
            .await?
            .text()
            .await?;

        let sha = raw_commit
            .lines()
            .find(|line| line.starts_with("From "))
            .unwrap()
            .split_whitespace()
            .nth(1)
            .ok_or(anyhow!("Could not find commit SHA"))?;

        Ok(sha.to_owned())
    }

    pub async fn get_toolchain_channel() -> anyhow::Result<String> {
        let raw_toml_file = reqwest::get(
            "https://raw.githubusercontent.com/embassy-rs/embassy/main/rust-toolchain.toml",
        )
        .await?
        .text()
        .await?;

        let toolchain_file: RustToolchain = toml::from_str(&raw_toml_file)?;

        Ok(toolchain_file._toolchain._channel)
    }

    pub async fn get_crate_version(name: impl Into<String>) -> anyhow::Result<String> {
        let name: String = name.into();

        // exceptions:
        // - "embassy-boot-*":
        //      crates are not in the "embassy-boot" directory,
        //      eg: dir for "embassy-boot-stm32" is "./embassy-boot/stm32"
        let path = if name.starts_with("embassy-boot-") {
            let subdir = name.split('-').nth(2).unwrap();
            format!("embassy-boot/{}/Cargo.toml", subdir)
        } else {
            format!("{}/Cargo.toml", name)
        };

        let raw_content = reqwest::get(&format!(
            "https://raw.githubusercontent.com/embassy-rs/embassy/main/{}",
            path
        ))
        .await?
        .text()
        .await?;

        let cargo_file: CrateManifest = toml::from_str(&raw_content)?;

        Ok(cargo_file._package._version)
    }
}

#[derive(serde::Deserialize)]
struct RustToolchain {
    #[serde(rename = "toolchain")]
    _toolchain: RustToolchainInner,
}

#[derive(serde::Deserialize)]
struct RustToolchainInner {
    #[serde(rename = "channel")]
    _channel: String,
    #[serde(flatten)]
    _other: Table,
}

#[derive(serde::Deserialize)]
struct CrateManifest {
    #[serde(rename = "package")]
    _package: CrateManifestPackage,
}

#[derive(serde::Deserialize)]
struct CrateManifestPackage {
    #[serde(rename = "name")]
    _name: String,
    #[serde(rename = "version")]
    _version: String,
}
