use config_tools::Config;

#[test]
fn test_config_builder_general() {
    let config = Config::builder().general().set("key1", "value1").build();
    assert!(
        config.general_values.contains_key("key1"),
        "General values should be stored without a section."
    );

    assert_eq!(
        config.general_values.get("key1"),
        Some(&"value1".to_string()),
        "Key1 should equal 'value1'"
    );

    assert!(
        config.sections.is_empty(),
        "Sections should be empty when using general values."
    )
}

#[test]
fn test_config_builder_sectioned() {
    let config = Config::builder()
        .section("Application")
        .set("host", "localhost")
        .build();

    assert_eq!(
        config.get(Some("Application"), "host"),
        Some(&"localhost".to_string()),
        "Host should equal 'localhost'"
    );
}

#[test]
fn test_config_builder_update() {
    let mut config = Config::builder()
        .section("Application")
        .set("host", "localhost")
        .build();

    config.update(Some("Application"), "host", "0.0.0.0");

    assert_eq!(
        config.get(Some("Application"), "host"),
        Some(&"0.0.0.0".to_string()),
        "Host should now equal '0.0.0.0'"
    );
}

#[test]
fn test_default_config_loading() {
    use config_tools::sectioned_defaults;
    let config = Config::load_or_default("nonexistent.ini", || sectioned_defaults! {
        {
            "console" => "true",
            "log_level" => "info",
        }
        ["Server"] {
            "address" => "127.0.0.1",
            "port" => "8080",
            "threads" => "4",
        }
    });

    let console = config.get_as::<bool>(None, "console").unwrap();
    let log_level = config.get(None, "log_level").unwrap();
    assert_eq!(console, true);
    assert_eq!(log_level, "info");

    let address = config.get(Some("Server"), "address").unwrap();
    let port = config.get_as::<u16>(Some("Server"), "port").unwrap();
    let threads = config.get_as::<u16>(Some("Server"), "threads").unwrap();
    assert_eq!(address, "127.0.0.1");
    assert_eq!(port, 8080);
    assert_eq!(threads, 4);
}

#[test]
fn test_default_config_loading_with_missing_keys() {
    use config_tools::sectioned_defaults;
    let config = Config::load_or_default("nonexistent.ini", || sectioned_defaults! {
        {
            "console" => "true",
            "log_level" => "info",
        }
        ["Server"] {
            "address" => "127.0.0.1",
            "port" => "8080",
            "threads" => "4",
        }
    });

    // Try to access non-existent key
    assert!(config.get(None, "missing_key").is_none(), "Should return None for missing key");

    // Try to access non-existent section
    assert!(config.get(Some("NonExistentSection"), "any_key").is_none(), "Should return None for missing section");
}


#[test]
fn test_get_as_type_mismatch() {
    let config = Config::builder().general().set("key1", "value1").build();

    let result = config.get_as::<u16>(None, "key1"); // Attempt to parse a string into u16
    assert!(result.is_none(), "get_as should return None on type mismatch");
}