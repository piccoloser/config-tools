# config_tools

`config_tools` is a lightweight, ergonomic Rust library for working with INI-style configuration files. It offers:

* A builder pattern for programmatic config creation
* Macros for concise default declarations
* Optional typed section parsing using `FromSection`
* Graceful fallbacks with `load_or_default_outcome`
* `serde` support for full serialization and deserialization

It is built on top of [`rust-ini`](https://github.com/zonyitoo/rust-ini) and designed for developer ergonomics first.

---

## Quickstart

```rust
use config_tools::{Config, sectioned_defaults};

let outcome = Config::load_or_default_outcome(
    "config.ini",
    sectioned_defaults! {
        { "debug" => "true" }
        
        ["App"] {
            "threads" => "4"
        }
    }
);

if outcome.used_default() {
    eprintln!("Using fallback config.");
}

let config = outcome.into_inner();
```

---

## Manual Configuration

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

---

## Macros for Inline Defaults

```rust
use config_tools::{sectioned_defaults, general_defaults, Config};

let sectioned: Config = sectioned_defaults! {
    { "logging" => "true" }

    ["Server"] {
        "host" => "127.0.0.1",
        "port" => "8080"
    }
};

let general: Config = general_defaults! {
    "console" => "true",
    "logging" => "true",
};
```

---

## Loading and Saving Configs

```rust
use config_tools::Config;

let config = Config::load("config.ini")?;
config.save("out.ini")?;
```

You can also handle missing files gracefully:

```rust
let default = Config::builder().set("fallback", "true").build();
let config = Config::load_or_default("config.ini", default);
```

Or check whether defaults were used:

```rust
let outcome = Config::load_or_default_outcome("config.ini", Config::default());

if outcome.used_default() {
    println!("File not found; using defaults.");
}

let config = outcome.into_inner();
```

---

## Typed Section Parsing with `FromSection`

```rust
use config_tools::{Config, FromSection};

#[derive(FromSection)]
struct ServerConfig {
    host: String,
    port: u16,
}

let config = Config::load("config.ini")?;
let server_section = config.section("Server").unwrap();
let server: ServerConfig = ServerConfig::from_section(server_section)?;
```

---

## `Config` API

* `Config::builder()`: Starts a new builder
* `Config::load(path)`: Loads from file
* `Config::save(path)`: Saves to file
* `Config::load_or_default(path, default)`: Uses a fallback if loading fails
* `Config::load_or_default_outcome(...)`: Same as above, but returns `LoadOutcome`
* `config.get(section, key)`: Returns a value as `Option<String>`
* `config.get_as::<T>(...)`: Parses value into a type
* `config.update(...)`: Updates or inserts a key-value pair

---

## `LoadOutcome`

Returned from `load_or_default_outcome`:

* `LoadOutcome::FromFile(config)`
* `LoadOutcome::FromDefault(config)`

### Methods:

* `.into_inner()`: Extract the config
* `.as_ref()`, `.as_mut()`: Borrow access
* `.used_default() -> bool`: Did fallback occur?

---

## Macros

### `sectioned_defaults!`

```rust
let config = sectioned_defaults! {
    { "logging" => "true" }

    ["App"] {
        "theme" => "dark"
    }
};
```

Supports variables for section names, keys, and values (must be strings). General keys must come first.

---

### `general_defaults!`

```rust
let config = general_defaults! {
    "logging" => "true",
    "console" => "true"
};
```

---

## Procedural Macro: `#[derive(FromSection)]`

Allows typed parsing of section contents:

```rust
#[derive(FromSection)]
struct MySettings {
    path: String,
    enabled: bool,
}
```

Fields must implement `FromStr`.