use base64::{engine::general_purpose, Engine as _};
use clap_stdin::MaybeStdin;
use urlencoding;

pub fn encode(text: MaybeStdin<String>) -> String {
	general_purpose::STANDARD.encode(text.as_bytes())
}

pub fn decode(text: MaybeStdin<String>) -> Result<String, String> {
	match general_purpose::STANDARD.decode(text.as_bytes()) {
		Ok(bytes) => Ok(String::from_utf8_lossy(&bytes).to_string()),
		Err(err) => Err(format!("Failed to decode base64 string: {}", err)),
	}
}

pub fn base64url_encode(text: MaybeStdin<String>) -> String {
	general_purpose::URL_SAFE_NO_PAD.encode(text.as_bytes())
}

pub fn base64url_decode(text: MaybeStdin<String>) -> Result<String, String> {
	match general_purpose::URL_SAFE_NO_PAD.decode(text.as_bytes()) {
		Ok(bytes) => Ok(String::from_utf8_lossy(&bytes).to_string()),
		Err(err) => Err(format!("Failed to decode base64url string: {}", err)),
	}
}

pub fn url_encode(text: MaybeStdin<String>) -> String {
	urlencoding::encode(&text).to_string()
}

pub fn url_decode(text: MaybeStdin<String>) -> Result<String, String> {
	match urlencoding::decode(&text) {
		Ok(decoded) => Ok(decoded.to_string()),
		Err(err) => Err(format!("Failed to decode URL: {}", err)),
	}
}
