use crate::color::Color;
use serde::Deserialize;
use std::fs;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Scheme {
    name: String,
    color: HashMap<String, Vec<u8>>
}

impl Scheme {
    pub fn from_path(path: String) -> Result<Scheme, serde_yaml::Error> {
        let raw: String = fs::read_to_string(path)
            .expect("failed to read file");
        let scheme: Scheme = serde_yaml::from_str(&raw)
            .expect("failed to deserialize file");

        Ok(scheme)
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn color(&self, key: String) -> Color {
        let vec: &Vec<u8> = self.color.get(&key).unwrap();
        Color::from_vec(vec)
    }
}
