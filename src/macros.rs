#[macro_export]
/// Generate a `Config` object with sections and default values.  
/// Variables are supported in section, key, and value fields, but must be strings.
/// # Syntax
/// ```rust
/// # use config_tools::{sectioned_defaults, Config};
/// let server_section = "Server";  // Variables are supported
/// let default_host = "127.0.0.1"; // across all fields.
/// let secure = "true";            // Value must be a string!
/// 
/// let mut config: Config = sectioned_defaults! {
/// // Optional general section (no section title)
/// // must be the first section if included
/// {
///     "console" => "true",
/// }
/// 
/// // Section titles are enclosed in square brackets
/// [server_section] {
///     "host" => default_host,
///     "port" => "8080",
/// }
///
/// ["Window"] {
///     "width" => "720",
///     "height" => "480",
/// }
/// };
/// ```
macro_rules! sectioned_defaults {
    (
        {
            $($general_key:expr => $general_value:expr),* $(,)?
        }
        $(
        [$section:expr] {
            $($key:expr => $value:expr),* $(,)?
        }
    )*) => {
        config_tools::Config::new()
        $(.set($general_key, $general_value))*
        $(
            .section($section)
            $(.set($key, $value))*
        )*
        .build();
    };
    ($(
        [$section:expr] {
            $($key:expr => $value:expr),* $(,)?
        }
    )*) => {
        config_tools::Config::new()
        $(
            .section($section)
            $(.set($key, $value))*
        )*
        .build();
    }
}

#[macro_export]
/// Generate a `Config` object with default values in a section entitled "DEFAULT".  
/// Variables are supported in key and value fields, but must be strings.
/// # Syntax
/// ```rust
/// # use config_tools::{general_defaults, Config};
/// let logging = "logging";    // Variables are supported across all fields.
/// let use_logging = "true";   // Value must be a string!
/// let mut config: Config = general_defaults! {
///    "console" => "true",
///    logging => use_logging,
/// };
/// ```
macro_rules! general_defaults {
    ($($key:expr => $value:expr),* $(,)?) => {
        config_tools::Config::new()
            $(.set($key, $value))*
            .build();
    }
}
