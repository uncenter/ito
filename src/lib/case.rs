use clap_stdin::MaybeStdin;

use convert_case::{Case, Casing};

pub fn upper(text: MaybeStdin<String>) -> String {
	return text.to_case(Case::Upper);
}

pub fn lower(text: MaybeStdin<String>) -> String {
	return text.to_case(Case::Lower);
}

pub fn title(text: MaybeStdin<String>) -> String {
	return text.to_case(Case::Title);
}

pub fn alternating(text: MaybeStdin<String>) -> String {
	return text.to_case(Case::Alternating);
}

pub fn camel(text: MaybeStdin<String>) -> String {
	return text.to_case(Case::Camel);
}

pub fn pascal(text: MaybeStdin<String>) -> String {
	return text.to_case(Case::Pascal);
}

pub fn snake(text: MaybeStdin<String>) -> String {
	return text.to_case(Case::Snake);
}

pub fn screaming(text: MaybeStdin<String>) -> String {
	return text.to_case(Case::ScreamingSnake);
}

pub fn kebab(text: MaybeStdin<String>) -> String {
	return text.to_case(Case::Kebab);
}
