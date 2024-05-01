use clap_stdin::MaybeStdin;

pub fn join(text: Vec<String>, joiner: String) -> String {
	return text.join(&joiner);
}

pub fn reverse(text: MaybeStdin<String>) -> String {
	return text.chars().rev().collect::<String>();
}

pub fn repeat(text: MaybeStdin<String>, count: usize) -> String {
	return text.repeat(count);
}

pub fn replace(text: MaybeStdin<String>, find: String, replace: String) -> String {
	return text.replace(&find, &replace);
}

pub fn trim(text: MaybeStdin<String>) -> String {
	return text.trim().to_string();
}
