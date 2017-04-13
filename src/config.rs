pub struct Config {
    /// Optional text to output at the beginning of the file
    pub file_header: Option<String>,
    /// Optional text to output at the end of the file
    pub file_trailer: Option<String>,
    /// Optional text to output at major sections to deter manual editting
    pub file_autogen_warning: Option<String>,
    /// Optional text to output before each function declaration
    pub function_prefix: Option<String>,
    /// Optional text to output after each function declaration
    pub function_postfix: Option<String>,
    /// Whether to add a `Sentinel` value at the end of every enum
    /// This is useful in Gecko for IPC serialization
    pub enum_add_sentinel: bool,
}

impl Config {
    pub fn default() -> Config {
        Config {
            file_header: None,
            file_trailer: None,
            file_autogen_warning: None,
            function_prefix: None,
            function_postfix: None,
            enum_add_sentinel: false,
        }
    }

    pub fn gecko_webrender() -> Config {
        let license = r###"/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */"###;

        let autogen = "/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen. */";

        Config {
            file_header: Some(String::from(license)),
            file_trailer: None,
            file_autogen_warning: Some(String::from(autogen)),
            function_prefix: Some(String::from("WR_INLINE")),
            function_postfix: Some(String::from("WR_FUNC")),
            enum_add_sentinel: true,
        }
    }

    pub fn load(config: &str) -> Option<Config> {
        match config {
            "default" => Some(Config::default()),
            "wr" => Some(Config::gecko_webrender()),
            _ => None,
        }
    }
}
