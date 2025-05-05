use crate::{builder::ConfigBuilder, error::Error, outcome::LoadOutcome};
use ini::Ini;
use std::{collections::BTreeMap, path::Path};

pub trait Section: Sized {
    fn from_section(map: &BTreeMap<String, String>) -> Result<Self, Error>;
}

/// Represents an INI-style configuration, including both general
/// values (not tied to any section) and sectioned key-value pairs.
///
/// You can build a `Config` manually using the [`ConfigBuilder`] API,
/// load one from a file, or create defaults using macros like
/// [`crate::sectioned_defaults!`] and [`crate::general_defaults!`].
#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub sections: BTreeMap<String, BTreeMap<String, String>>,
    pub general_values: BTreeMap<String, String>,
}

impl Config {
    pub fn general(&self) -> &BTreeMap<String, String> {
        &self.general_values
    }

    pub fn get(&self, section: Option<&str>, key: &str) -> Option<String> {
        if let Some(section) = section {
            return self.sections.get(section).and_then(|s| s.get(key)).cloned();
        } else {
            return self.general_values.get(key).cloned();
        }
    }

    pub fn get_as<T>(&self, section: Option<&str>, key: &str) -> Option<T>
    where
        T: std::str::FromStr + std::fmt::Debug,
    {
        self.get(section, key).and_then(|v| v.parse().ok())
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let ini = Ini::load_from_file(path).map_err(Error::ConfigLoad)?;
        let mut sections = BTreeMap::new();
        let mut general_values = BTreeMap::new();

        for (section, prop) in ini.iter() {
            if let Some(section) = section {
                let mut section_map = BTreeMap::new();
                prop.iter().for_each(|(key, value)| {
                    section_map.insert(key.to_string(), value.to_string());
                });

                sections.insert(section.to_string(), section_map);
            } else {
                prop.iter().for_each(|(key, value)| {
                    general_values.insert(key.to_string(), value.to_string());
                })
            }
        }

        Ok(Config {
            sections,
            general_values,
        })
    }

    pub fn load_or_default<P: AsRef<Path>>(path: P, default: Config) -> Self {
        match Self::load(path) {
            Ok(config) => config,
            Err(_) => default,
        }
    }

    pub fn load_or_default_outcome<P: AsRef<Path>>(path: P, default: Config) -> LoadOutcome {
        match Self::load(path) {
            Ok(config) => LoadOutcome::FromFile(config),
            Err(_) => LoadOutcome::FromDefault(default),
        }
    }

    pub fn builder() -> ConfigBuilder {
        ConfigBuilder {
            config: Config::default(),
            section: None,
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<&Self, Error> {
        let mut ini = Ini::new();

        let mut section = ini.with_general_section();
        for (key, value) in &self.general_values {
            section.set(key, value);
        }

        for (title, prop) in &self.sections {
            let mut section = ini.with_section(Some(title));
            for (key, value) in prop {
                section.set(key, value);
            }
        }

        ini.write_to_file(path).map_err(Error::ConfigCreation)?;

        Ok(self)
    }

    pub fn section(&self, title: &str) -> Option<&BTreeMap<String, String>> {
        self.sections.get(title)
    }

    pub fn sections(&self) -> &BTreeMap<String, BTreeMap<String, String>> {
        &self.sections
    }

    pub fn update(&mut self, section: Option<&str>, key: &str, value: &str) -> &mut Self {
        if let Some(section) = section {
            self.sections
                .entry(section.to_string())
                .or_default()
                .insert(key.to_string(), value.to_string());
        } else {
            self.general_values
                .insert(key.to_string(), value.to_string());
        }

        self
    }
}
