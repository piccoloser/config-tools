use config_tools::{Config, sectioned_defaults};

#[test]
fn test_load_outcome_used_default() {
    let default = sectioned_defaults! {
        {
            "debug" => "true"
        }
    };

    let outcome = Config::load_or_default_outcome("nonexistent_file.ini", default.clone());

    assert!(outcome.used_default(), "Should detect that default was used");
    assert_eq!(outcome.as_ref(), &default, "Should match the default config");
}

#[test]
fn test_load_outcome_mutation() {
    let mut outcome = Config::load_or_default_outcome(
        "nonexistent_file.ini",
        Config::default(),
    );

    outcome.as_mut().update(None, "key", "value");
    assert_eq!(outcome.as_ref().get(None, "key"), Some("value".to_string()));
}