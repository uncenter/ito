// Original implemenation written by Ron Li at https://github.com/magiclen/words-count.

// MIT License

// Copyright (c) 2020 magiclen.org (Ron Li)

use unicode_blocks::{CJK_SYMBOLS_AND_PUNCTUATION, HALFWIDTH_AND_FULLWIDTH_FORMS};

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct WordsCount {
	pub words: Vec<String>,
	pub characters: usize,
	pub whitespaces: usize,
	pub cjk: usize,
}

/// Given a character, determine whether it is a non-word CJK character.
pub fn is_cjk_other(c: char) -> bool {
	return CJK_SYMBOLS_AND_PUNCTUATION.contains(c) || HALFWIDTH_AND_FULLWIDTH_FORMS.contains(c);
}

/// Count the words in the given string. In general, every non-CJK string of characters between two whitespaces is a word. Dashes (at least two dashes) are word limit, too. A CJK character is considered to be an independent word.
pub fn count_words<S: AsRef<str>>(s: S) -> WordsCount {
	let mut in_word = false;
	let mut consecutive_dashes = 0usize;

	let mut count = WordsCount::default();
	let mut word = String::new();

	for c in s.as_ref().chars() {
		count.characters += 1;
		word.push_str(&c.to_string());

		if c.is_whitespace() {
			consecutive_dashes = 0;

			count.whitespaces += 1;

			if in_word {
				count.words.push(word);
				word = String::new();

				in_word = false;
			}
		} else {
			match c {
				'-' => {
					consecutive_dashes += 1;

					if consecutive_dashes > 1 && in_word {
						if consecutive_dashes == 2 {
							count.words.push(word);
							word = String::new();
						}

						in_word = false;

						continue;
					}
				}
				_ => {
					consecutive_dashes = 0;

					if is_cjk_other(c) {
						count.cjk += 1;
						word = String::new();

						continue;
					} else if unicode_blocks::is_cjk(c) {
						count.words.push(word);
						word = String::new();
						count.cjk += 1;

						if in_word {
							count.words.push(word.clone());

							in_word = false;
						}

						continue;
					}
				}
			}

			if !in_word {
				in_word = true;
			}
		}
	}

	if in_word {
		count.words.push(word);
	}

	return count;
}
