use crate::CreateCommand;

use self::{data::DATA, generate::GeneratorConfig};

mod data;
mod generate;
mod templates;

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
    };
    generate::create(config).await?;

    Ok(())
}
