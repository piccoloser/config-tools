use config_tools::{sectioned_defaults, Config};
use derive_macro::FromSection;

#[derive(Debug, FromSection, PartialEq)]
struct ServerSettings {
    address: String,
    port: u16,
    threads: u16,
}

#[test]
fn test_incomplete_section_parsing() {
    let config = Config::load_or_default("nonexistent.ini", || sectioned_defaults! {
        ["Server"] {
            "address" => "192.168.1.1",  // Missing `port` and `threads`
        }
    });

    let server_settings_result = ServerSettings::from_section(&config.section("Server").unwrap());
    
    assert!(
        server_settings_result.is_err(),
        "Parsing an incomplete section should result in an error"
    );
}


#[test]
fn test_section_parsing_into_struct() {
    let config = Config::load_or_default("nonexistent.ini", || sectioned_defaults! {
        ["Server"] {
            "address" => "192.168.1.1",
            "port" => "8000",
            "threads" => "8",
        }
    });

    // Parse section into a struct
    let server_settings = ServerSettings::from_section(&config.section("Server").unwrap()).unwrap();

    // Check that the values are correctly parsed into the struct
    let expected_settings = ServerSettings {
        address: "192.168.1.1".to_string(),
        port: 8000,
        threads: 8,
    };
    assert_eq!(server_settings, expected_settings);
}