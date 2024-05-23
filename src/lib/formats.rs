use clap_stdin::MaybeStdin;

use serde_json::Value as JSONValue;
use toml::Value as TOMLValue;

pub fn json_to_toml(text: MaybeStdin<String>) -> String {
	let value: TOMLValue = serde_json::from_str(&text).unwrap();
	return value.to_string();
}

pub fn toml_to_json(text: MaybeStdin<String>) -> String {
	let value: JSONValue = toml::from_str(&text).unwrap();
	return value.to_string();
}
