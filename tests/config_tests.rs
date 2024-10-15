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
