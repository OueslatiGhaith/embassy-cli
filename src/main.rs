use data::DATA;
use generate::{create, GeneratorConfig};

mod data;
mod generate;

fn main() -> anyhow::Result<()> {
    let name = inquire::Text::new("Project name").prompt()?;
    let vendor = inquire::Select::new("Select a vendor", DATA.vendor_list()).prompt()?;
    let mcu = inquire::Select::new("Select an MCU", DATA.mcu_list(&vendor)).prompt()?;
    let target = DATA.target(&mcu);

    let config = GeneratorConfig {
        name,
        vendor,
        mcu,
        target,
    };
    create(config)?;

    Ok(())
}
