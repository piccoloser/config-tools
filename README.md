# `config_tools`

## Overview

`config_tools` is a configuration management library designed for handling hierarchical configurations using sections and key-value pairs. It provides builders to customize and create `Config` objects, macros to simplify the creation of configuration files, and error handling for configuration loading and saving.

`config_tools` is built on top of [rust-ini](https://github.com/zonyitoo/rust-ini) and focuses mostly on convenience.

## Structs

### `Config`

#### Traits

1. `Debug` and `Default`
2. From [serde](https://serde.rs/derive.html): `Deserialize` and `Serialize`

Represents the entire configuration, with support for both general (non-sectioned) values and sectioned values.

-   **Fields**:

    -   `sections`: A `BTreeMap<String, BTreeMap<String, String>>` where each key is the section title, and the values are the key-value pairs for that section.
    -   `general_values`: A `BTreeMap<String, String>` that stores key-value pairs not tied to a specific section.

-   **Methods**:
    -   `general(&self) -> &BTreeMap<String, String>`: Returns a reference to the general section.
    -   `get(section: Option<&str>, key: &str) -> Option<&String>`: Retrieves a value from a specific section or from the general section if no section is provided.
    -   `get_as<T>(&self, section: Option<&str>, key: &str) -> Option<T>`: Retrieve a value from a specific section or from the general section if no section is provided, parsing said value into a given type `T` so long as the type implements `std::str::FromStr` and `std::fmt::Debug`.
    -   `load(filename: &str) -> Result<Self, Error>`: Loads a configuration from an `.ini` file.
    -   `load_or_default(filename: &str, default: Config) -> Self`: Loads a configuration from an `.ini` file or uses a provided default.
    -   `builder() -> ConfigBuilder`: Starts the creation of a new configuration with a builder.
    -   `save(&self, filename: &str) -> Result<&Self, Error>`: Saves the current configuration to an `.ini` file.
    -   `section(title: &str) -> Option<&BTreeMap<String, String>>`: Retrieves a given section from the configuration or `None`.
    -   `sections() -> &BTreeMap<String, BTreeMap<String, String>>`: Retrieves the section map of the configuration.
    -   `update(&mut self, section: Option<&str>, key: &str, value: &str) -> &mut Self`: Updates or adds a key-value pair to a specific section or general configuration.

### `ConfigBuilder`

A builder pattern for creating and customizing `Config` objects before finalizing them.

-   **Methods**:
    -   `general() -> Self`: Specifies that the builder is targeting the general section (no specific section).
    -   `section(title: &str) -> Self`: Specifies a section to set key-value pairs in.
    -   `set(key: &str, value: &str) -> Self`: Sets a key-value pair in the current section or general section.
    -   `build() -> Config`: Finalizes and returns the built `Config` object.

---

## Enums

### `Error`

Defines the possible errors that can occur during the use of the crate.

-   **Variants**:

    -   `AlreadyExists`: Returned when a key already exists in a configuration.
    -   `NotFound`: Returned when a key is not found in the configuration.
    -   `ConfigLoad(ini::Error)`: Error variant for failures during loading of `.ini` files.
    -   `ConfigCreation(std::io::Error)`: Error variant for issues during the saving of `.ini` files.
    -   `ConfigParse(String)`: Error variant for issues encountered during parsing of configuration values.

-   **Trait Implementation**:
    -   `fmt::Display`: Custom error message formatting for each error variant.

---

## Macros

### `sectioned_defaults!`

Generates a `Config` object with support for sections and default values.

-   **Syntax**:

    ```rust
    let mut config: Config = sectioned_defaults! {
        { "console" => "true" },    // General section

        ["Server"] {                // Section with title
            "host" => "127.0.0.1",
            "port" => "8080",
        },

        ["Window"] {
            "width" => "720",
            "height" => "480",
        }
    };
    ```

-   **Notes**:
    -   Supports variables for section names, keys, and values as long as they are strings.
    -   General key-value pairs must be specified first if included.

### `general_defaults!`

Generates a `Config` object with default values in a general section.

-   **Syntax**:
    ```rust
    let mut config: Config = general_defaults! {
        "console" => "true",
        "logging" => "true",
    };
    ```
-   **Notes**:
    -   The keys and values must be strings.
    -   This macro is focused on generating default configurations without specific sections.

---

## Procedural Macros

### `FromSection`

This procedural macro derives an implementation of the `Section` trait for a struct, enabling automatic parsing from a `BTreeMap<String, String>`.

-   **Syntax**:

    ```rust
    #[derive(FromSection)]
    struct ServerConfig {
        host: String,
        port: u16,
    }
    ```

-   **Notes**:
    -   The fields of the struct must implement `FromStr`, and the macro will automatically attempt to parse each field from the corresponding string value in the section.

---

# Usage Examples

## Manually Creating a New Configuration

```rust
use config_tools::Config;

let config = Config::builder()
    .general()
    .set("logging", "true")
    .set("verbose", "false")
    .section("Database")
    .set("host", "localhost")
    .set("port", "5432")
    .build();
```

## Creating Configurations Using Macros

```rust
use config_tools::{sectioned_defaults, general_defaults, Config};

// Using sectioned_defaults! macro
let sectioned_config: Config = sectioned_defaults! {
    { "logging" => "true" },  // General section
    ["Server"] { "host" => "127.0.0.1", "port" => "8080" }
};

// Using general_defaults! macro
let general_config: Config = general_defaults! {
    "console" => "true",
    "logging" => "true",
};
```

## Parsing Sections into Structs with `FromSection`

```rust
use config_tools::{Config, ServerConfig};

#[derive(FromSection)]
struct ServerConfig {
    host: String,
    port: u16,
}

let config = Config::load("config.ini")?;
let server_section = config.section("Server").unwrap();
let server_config = ServerConfig::from_section(server_section)?;

println!("{:?}", server_config);
```

## Updating a Configuration

```rust
use config_tools::{sectioned_defaults, Config};

let mut config = sectioned_defaults! {
    {
        "logging" => "true",
        "verbose" => "false",
    }

    ["Database"] {
        "host" => "localhost",
        "port" => "5432",
    }
}

config.update(None, "verbose", "true");
config.update(Some("Database"), "port", "3306");
```

## Loading and Saving Configurations

```rust
use config_tools::Config;

fn main() -> Result<(), config_tools::Error> {
    // Load config from file
    let config = Config::load("config.ini")?;

    // Access a value
    if let Some(host) = config.get(Some("Database"), "host") {
        println!("Database host: {}", host);
    }

    // Save the config
    config.save("new_config.ini")?;

    Ok(())
}
```

---

## Error Handling

All configuration operations return `Result` types that include the custom `Error` enum, which provides more specific details about the nature of failures, such as file I/O errors or missing keys.
