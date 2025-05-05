use crate::Config;

/// The result of loading a configuration, indicating whether the config was
/// loaded from a file or constructed from a default fallback.
///
/// Use [`LoadOutcome::used_default`] to determine which case occurred,
/// or extract the inner config using [`LoadOutcome::into_inner`].
#[derive(Clone, Debug, PartialEq)]
pub enum LoadOutcome {
    FromDefault(Config),
    FromFile(Config),
}

impl LoadOutcome {
    #[must_use]
    pub fn into_inner(self) -> Config {
        match self {
            LoadOutcome::FromDefault(cfg) | LoadOutcome::FromFile(cfg) => cfg
        }
    }

    pub fn as_mut(&mut self) -> &mut Config {
        match self {
            LoadOutcome::FromDefault(cfg) | LoadOutcome::FromFile(cfg) => cfg
        }
    }

    pub fn as_ref(&self) -> &Config {
        match self {
            LoadOutcome::FromDefault(cfg) | LoadOutcome::FromFile(cfg) => cfg
        }
    }

    pub fn used_default(&self) -> bool {
        matches!(self, LoadOutcome::FromDefault(_))
    }
}