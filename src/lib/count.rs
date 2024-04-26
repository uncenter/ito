use clap_stdin::MaybeStdin;

use unicode_segmentation::UnicodeSegmentation;

pub fn count_lines(text: MaybeStdin<String>) -> String {
	return text.lines().count().to_string();
}

pub fn count_chars(text: MaybeStdin<String>, character: Option<char>) -> String {
	if let Some(character) = character {
		return text.chars().filter(|c| *c == character).count().to_string();
	} else {
		return text.chars().count().to_string();
	}
}

pub fn count_words(text: MaybeStdin<String>, word: Option<String>) -> String {
	let words = text.unicode_words().collect::<Vec<&str>>();
	if let Some(word) = word {
		return words
			.iter()
			.filter(|&w| *w.to_lowercase() == word.to_lowercase())
			.count()
			.to_string();
	} else {
		return words.len().to_string();
	}
}
