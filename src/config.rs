use crate::error::Error;
use ini::Ini;
use std::collections::BTreeMap;

pub struct ConfigBuilder<'a> {
    config: Config,
    section: Option<&'a str>,
}

impl<'a> ConfigBuilder<'a> {
    pub fn general(mut self) -> Self {
        self.section = None;
        self
    }

    pub fn section(mut self, title: &'a str) -> Self {
        self.section = Some(title);
        self.config.sections.entry(title.to_string()).or_default();
        self
    }

    pub fn set(mut self, key: &str, value: &str) -> Self {
        if let Some(section) = self.section {
            self.config
                .sections
                .entry(section.to_string())
                .or_default()
                .insert(key.to_string(), value.to_string());
        } else {
            self.config.general_values.insert(key.to_string(), value.to_string());
        }
        self
    }

    pub fn build(self) -> Config {
        self.config
    }
}

#[derive(Debug, Default)]
pub struct Config {
    pub sections: BTreeMap<String, BTreeMap<String, String>>,
    pub general_values: BTreeMap<String, String>,
}

impl Config {
    pub fn get(&self, section: Option<&str>, key: &str) -> Option<&String> {
        if let Some(section) = section {
            return self.sections.get(section).and_then(|s| s.get(key));
        } else {
            return self.general_values.get(key);
        }
    }

    pub fn load(filename: &str) -> Result<Self, Error> {
        let ini = Ini::load_from_file(filename).map_err(Error::ConfigLoad)?;
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

        Ok(Config { sections, general_values })
    }

    pub fn new<'a>() -> ConfigBuilder<'a> {
        ConfigBuilder {
            config: Config::default(),
            section: None,
        }
    }

    pub fn save(&self, filename: &str) -> Result<(), Error> {
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

        ini.write_to_file(filename).map_err(Error::ConfigCreation)
    }

    pub fn sections(&self) -> &BTreeMap<String, BTreeMap<String, String>> {
        &self.sections
    }

    pub fn update(&mut self, section: Option<&str>, key: &str, value: &str) {
        if let Some(section) = section {
            self.sections
                .entry(section.to_string())
                .or_default()
                .insert(key.to_string(), value.to_string());
        } else {
            self.general_values.insert(key.to_string(), value.to_string());
        }
    }
}
