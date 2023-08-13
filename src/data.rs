use serde::{Deserialize, Serialize};

lazy_static::lazy_static! {
    pub static ref DATA: Data = serde_json::from_str(include_str!("../data/mcu_list.json")).unwrap();
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

    pub fn mcu_list(&self, vendor: &str) -> Vec<String> {
        self.vendors
            .iter()
            .find(|v| v.name == vendor)
            .unwrap()
            .mcu_list
            .clone()
    }

    pub fn target(&self, mcu: &str) -> String {
        self.flavors
            .iter()
            .find(|f| {
                let re = regex::Regex::new(&f.regex).unwrap();
                re.is_match(mcu)
            })
            .unwrap()
            .target
            .clone()
    }
}
