use clap_stdin::MaybeStdin;

pub fn reverse(text: MaybeStdin<String>) -> String {
	return text.chars().rev().collect::<String>();
}

pub fn repeat(text: MaybeStdin<String>, count: usize) -> String {
	return text.repeat(count);
}

pub fn replace(text: MaybeStdin<String>, find: String, replace: String) -> String {
	return text.replace(&find, &replace);
}
