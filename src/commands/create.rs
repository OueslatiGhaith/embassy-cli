use clap::{builder::PossibleValue, Parser, ValueEnum};

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
    vendor: Option<Vendor>,
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Vendor {
    St,
    Nrf,
    Rp,
}

impl ValueEnum for Vendor {
    fn value_variants<'a>() -> &'a [Self] {
        &[Vendor::St, Vendor::Nrf, Vendor::Rp]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Vendor::St => PossibleValue::new("ST").aliases(["st", "stm", "stm32", "STM", "STM32"]),
            Vendor::Nrf => PossibleValue::new("Nordic").aliases(["nrf", "nRF", "NRF"]),
            Vendor::Rp => PossibleValue::new("Raspberry").aliases(["rp", "RP"]),
        })
    }
}

impl From<Vendor> for String {
    fn from(value: Vendor) -> Self {
        match value {
            Vendor::St => "ST",
            Vendor::Nrf => "Nordic",
            Vendor::Rp => "Raspberry",
        }
        .into()
    }
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
        Vendor::from_str(
            &inquire::Select::new("Select a vendor", DATA.vendor_list()).prompt()?,
            true,
        )
        .unwrap() // should be safe
    };

    let mcu = if let Some(mcu) = cmd.mcu {
        mcu
    } else {
        inquire::Select::new("Select an MCU", DATA.mcu_list(vendor)?).prompt()?
    };

    DATA.validate(vendor, &mcu)?;

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
