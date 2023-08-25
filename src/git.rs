use anyhow::anyhow;
use toml::Table;

pub struct Git;

impl Git {
    pub async fn get_latest_commit() -> anyhow::Result<String> {
        let gh = octocrab::instance();
        let repo = gh.repos("embassy-rs", "embassy");

        let latest_commit = repo
            .list_commits()
            .per_page(1)
            .send()
            .await?
            .items
            .first()
            .ok_or(anyhow!("no commits in repo"))?
            .sha
            .clone();

        Ok(latest_commit)
    }

    pub async fn get_toolchain_channel() -> anyhow::Result<String> {
        let gh = octocrab::instance();
        let repo = gh.repos("embassy-rs", "embassy");

        let content = repo
            .get_content()
            .path("rust-toolchain.toml")
            .send()
            .await?;

        let toolchain_file = content
            .items
            .first()
            .ok_or(anyhow!("no rust-toolchain.toml in repo"))?;

        let toolchain_file = toolchain_file
            .decoded_content()
            .ok_or(anyhow!("rust-toolchain.toml is not a valid UTF-8 file"))?;

        let toolchain_file: RustToolchain = toml::from_str(&toolchain_file)?;

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
