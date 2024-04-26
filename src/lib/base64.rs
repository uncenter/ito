use clap_stdin::MaybeStdin;

use base64::{engine::general_purpose, Engine as _};

pub fn encode(text: MaybeStdin<String>) -> String {
	return general_purpose::STANDARD.encode(text.as_bytes());
}
pub fn decode(text: MaybeStdin<String>) -> Result<String, String> {
	match general_purpose::STANDARD.decode(text.as_bytes()) {
		Ok(bytes) => Ok(String::from_utf8_lossy(&bytes).to_string()),
		Err(err) => Err(format!("Failed to decode base64 string: {}", err)),
	}
}
