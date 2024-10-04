use serde::{Deserialize, Serialize};

use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Variant {
    pub name: String,
    display: String,
    color: String,
    custom_command: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Framework {
    name: String,
    display: String,
    color: String,
    variants: Vec<Variant>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Frameworks {
    frameworks: Vec<Framework>,
}

#[allow(clippy::new_without_default)]
impl Frameworks {
    pub fn new() -> Self {
        let json_content = fs::read_to_string("src/data.json").expect("Failed to read data.json");
        let frameworks: Frameworks = serde_json::from_str(&json_content).unwrap();
        frameworks
    }

    pub fn get_root_frameworks_names(&self) -> Vec<String> {
        self.frameworks.iter().map(|f| f.name.clone()).collect()
    }

    pub fn get_variants_by_name(&self, name: &str) -> Option<&Vec<Variant>> {
        self.frameworks
            .iter()
            .find(|f| f.name == name)
            .map(|f| &f.variants)
    }
}
