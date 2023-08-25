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
