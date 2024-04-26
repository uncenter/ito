use clap_stdin::MaybeStdin;

use rand::seq::SliceRandom;

pub fn reverse_lines(text: MaybeStdin<String>) -> String {
	return text.lines().rev().collect::<Vec<_>>().join("\n");
}

pub fn shuffle_lines(text: MaybeStdin<String>) -> String {
	let mut lines: Vec<_> = text.lines().collect();
	lines.shuffle(&mut rand::thread_rng());
	return lines.join("\n");
}

pub fn sort_lines(text: MaybeStdin<String>) -> String {
	let mut lines: Vec<_> = text.lines().collect();
	lines.sort();
	return lines.join("\n");
}
