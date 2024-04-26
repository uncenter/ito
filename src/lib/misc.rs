use clap_stdin::MaybeStdin;

pub fn reverse(text: MaybeStdin<String>) -> String {
	return text.chars().rev().collect::<String>();
}

pub fn replace(text: MaybeStdin<String>, find: String, replace: String) -> String {
	return text.replace(&find, &replace);
}
