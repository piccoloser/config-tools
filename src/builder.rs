use crate::Config;

/// A builder for incrementally constructing a [`Config`] object.
///
/// Supports fluent-style API for setting values in the general section or
/// named sections. To finalize the configuration, call [`ConfigBuilder::build`].
pub struct ConfigBuilder {
    pub(crate) config: Config,
    pub(crate) section: Option<String>,
}

impl ConfigBuilder {
    pub fn build(self) -> Config {
        self.config
    }

    pub fn general(mut self) -> Self {
        self.section = None;
        self
    }

    pub fn section(mut self, title: &str) -> Self {
        self.section = Some(title.to_string());
        self.config.sections.entry(title.to_string()).or_default();
        self
    }

    pub fn set(mut self, key: &str, value: &str) -> Self {
        if let Some(section) = self.section.as_ref() {
            self.config
                .sections
                .entry(section.clone())
                .or_default()
                .insert(key.to_string(), value.to_string());
        } else {
            self.config
                .general_values
                .insert(key.to_string(), value.to_string());
        }

        self
    }
}
