use bcrypt::{hash, DEFAULT_COST};
use clap_stdin::MaybeStdin;
use md5 as md5_crate;
use sha1::Sha1;
use sha2::{Digest, Sha256, Sha512};

pub fn bcrypt(text: MaybeStdin<String>) -> String {
	hash(text.as_bytes(), DEFAULT_COST).unwrap()
}

pub fn md5(text: MaybeStdin<String>) -> String {
	format!("{:x}", md5_crate::compute(text.as_bytes()))
}

pub fn sha1(text: MaybeStdin<String>) -> String {
	let mut hasher = Sha1::new();
	hasher.update(text.as_bytes());
	format!("{:x}", hasher.finalize())
}

pub fn sha256(text: MaybeStdin<String>) -> String {
	let mut hasher = Sha256::new();
	hasher.update(text.as_bytes());
	format!("{:x}", hasher.finalize())
}

pub fn sha512(text: MaybeStdin<String>) -> String {
	let mut hasher = Sha512::new();
	hasher.update(text.as_bytes());
	format!("{:x}", hasher.finalize())
}
