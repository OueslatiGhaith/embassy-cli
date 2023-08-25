use serde::{Deserialize, Serialize};

lazy_static::lazy_static! {
    pub static ref DATA: Data = serde_json::from_str(include_str!("../../data/mcu_list.json")).unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    vendors: Vec<Vendor>,
    flavors: Vec<Flavor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vendor {
    name: String,
    mcu_list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Flavor {
    regex: String,
    target: String,
}

// const DATA: Data = serde_json::from_str(include!("../data/mcu_list.json"));

impl Data {
    pub fn vendor_list(&self) -> Vec<String> {
        self.vendors
            .iter()
            .map(|v| v.name.clone())
            .collect::<Vec<String>>()
    }

    pub fn mcu_list(&self, vendor: &str) -> anyhow::Result<Vec<String>> {
        for v in &self.vendors {
            if v.name == vendor {
                return Ok(v.mcu_list.clone());
            }
        }

        anyhow::bail!("No MCU list found for vendor: {}", vendor);
    }

    pub fn target(&self, mcu: &str) -> anyhow::Result<String> {
        for flavor in &self.flavors {
            let re = regex::Regex::new(&flavor.regex)?;
            if re.is_match(mcu) {
                return Ok(flavor.target.clone());
            }
        }

        anyhow::bail!("No target found for MCU: {}", mcu);
    }

    pub fn validate(&self, vendor: &str, mcu: &str) -> anyhow::Result<()> {
        if !self.vendor_list().contains(&vendor.to_owned()) {
            anyhow::bail!("Invalid vendor: {}", vendor);
        }

        if !self.mcu_list(vendor)?.contains(&mcu.to_owned()) {
            anyhow::bail!("Invalid MCU: {}", mcu);
        }

        Ok(())
    }
}

/// list of supported embassy crates
pub enum EmbassyCrates {
    Executor,
    Futures,
    Lora,
    Nrf,
    Rp,
    Stm32Wpan,
    Stm32,
    Sync,
    Time,
}

impl From<EmbassyCrates> for String {
    fn from(val: EmbassyCrates) -> Self {
        match val {
            EmbassyCrates::Executor => "embassy-executor",
            EmbassyCrates::Futures => "embassy-futures",
            EmbassyCrates::Lora => "embassy-lora",
            EmbassyCrates::Nrf => "embassy-nrf",
            EmbassyCrates::Rp => "embassy-rp",
            EmbassyCrates::Stm32Wpan => "embassy-stm32-wpan",
            EmbassyCrates::Stm32 => "embassy-stm32",
            EmbassyCrates::Sync => "embassy-sync",
            EmbassyCrates::Time => "embassy-time",
        }
        .to_owned()
    }
}

impl EmbassyCrates {
    pub fn default_crates() -> Vec<Self> {
        [Self::Executor, Self::Time, Self::Sync, Self::Futures].into()
    }
}
