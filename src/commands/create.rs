use clap::Parser;

use crate::generator::{
    data::DATA,
    generate::{self, GeneratorConfig},
};

#[derive(Parser)]
pub struct CreateCommand {
    /// Project name
    #[clap(short, long)]
    name: Option<String>,
    /// Vendor
    #[clap(short, long)]
    vendor: Option<String>,
    /// MCU
    #[clap(short, long)]
    mcu: Option<String>,
    /// Do not pin to the latest commit of the Embassy crate
    #[clap(long)]
    no_pin: bool,
    /// Create project in a workspace
    #[clap(long)]
    workspace: bool,
}

pub async fn create(cmd: CreateCommand) -> anyhow::Result<()> {
    let name = if let Some(name) = cmd.name {
        name
    } else {
        inquire::Text::new("Project name").prompt()?
    };

    let vendor = if let Some(vendor) = cmd.vendor {
        vendor
    } else {
        inquire::Select::new("Select a vendor", DATA.vendor_list()).prompt()?
    };

    let mcu = if let Some(mcu) = cmd.mcu {
        mcu
    } else {
        inquire::Select::new("Select an MCU", DATA.mcu_list(&vendor)?).prompt()?
    };

    DATA.validate(&vendor, &mcu)?;

    let target = DATA.target(&mcu)?;

    let config = GeneratorConfig {
        name,
        vendor,
        mcu,
        target,
        no_pin: cmd.no_pin,
        workspace: cmd.workspace,
    };
    generate::create(config).await?;

    Ok(())
}
