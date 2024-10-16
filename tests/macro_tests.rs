#[macro_use]
extern crate config_tools;

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
fn test_mixed_defaults_macro() {
    let config = sectioned_defaults!(
        {
            "general_key1" => "general_value1"
        }
        ["section1"] {
            "section_key1" => "section_value1"
        }
    );

    // General section checks
    assert_eq!(config.get(None, "general_key1"), Some(&"general_value1".to_string()));

    // Section checks
    assert_eq!(config.get(Some("section1"), "section_key1"), Some(&"section_value1".to_string()));
}


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
fn test_sectioned_defaults_macro_with_missing_section_or_key() {
    let config = sectioned_defaults!(
        ["section1"] {
            "key1" => "value1",
        }
    );

    // Missing section
    assert!(config.get(Some("missing_section"), "key1").is_none(), "Should return None for missing section");

    // Missing key in existing section
    assert!(config.get(Some("section1"), "missing_key").is_none(), "Should return None for missing key in section");
}


#[test]
fn test_sectioned_defaults_macro_with_type_parsing() {
    let config = sectioned_defaults!(
        ["section1"] {
            "key1" => "100",
            "key2" => "true",
        }
    );

    // Type parsing with `get_as`
    let key1: u16 = config.get_as(Some("section1"), "key1").unwrap();
    let key2: bool = config.get_as(Some("section1"), "key2").unwrap();

    assert_eq!(key1, 100);
    assert_eq!(key2, true);
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