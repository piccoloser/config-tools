#[macro_use]
extern crate config_tools;

#[test]
fn test_sectioned_defaults_macro() {
    let config = sectioned_defaults!(
        ["section1"] {
            "key1" => "value1",
            "key2" => "value2",
        }
        ["section2"] {
            "key3" => "value3"
        }
    );

    assert_eq!(config.get(Some("section1"), "key1"), Some(&"value1".to_string()));
    assert_eq!(config.get(Some("section1"), "key2"), Some(&"value2".to_string()));
    assert_eq!(config.get(Some("section2"), "key3"), Some(&"value3".to_string()));
    assert!(config.get(Some("section2"), "key4").is_none());
}

#[test]
fn test_general_defaults_macro() {
    let config = general_defaults!(
        "key1" => "value1",
        "key2" => "value2",
    );

    assert_eq!(config.get(None, "key1"), Some(&"value1".to_string()));
    assert_eq!(config.get(None, "key2"), Some(&"value2".to_string()));
    assert!(config.get(None, "key3").is_none());
}

#[test]
fn test_sectioned_defaults_macro_with_general_section() {
    let config = sectioned_defaults!(
        {
            "key1" => "value1",
            "key2" => "value2",
        }
        ["section1"] {
            "key3" => "value3"
        }
    );

    assert_eq!(config.get(None, "key1"), Some(&"value1".to_string()));
    assert_eq!(config.get(None, "key2"), Some(&"value2".to_string()));
    assert_eq!(config.get(Some("section1"), "key3"), Some(&"value3".to_string()));
    assert!(config.get(Some("section1"), "key4").is_none());
}