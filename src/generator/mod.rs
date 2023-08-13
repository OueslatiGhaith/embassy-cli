use self::{data::DATA, generate::GeneratorConfig};

mod data;
mod generate;

pub fn create(
    name: Option<String>,
    vendor: Option<String>,
    mcu: Option<String>,
) -> anyhow::Result<()> {
    let name = if let Some(name) = name {
        name
    } else {
        inquire::Text::new("Project name").prompt()?
    };

    let vendor = if let Some(vendor) = vendor {
        vendor
    } else {
        inquire::Select::new("Select a vendor", DATA.vendor_list()).prompt()?
    };

    let mcu = if let Some(mcu) = mcu {
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
    };
    generate::create(config)?;

    Ok(())
}
