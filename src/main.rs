use clap::{Parser, Subcommand};
use clap_stdin::MaybeStdin;

use convert_case::{Case, Casing};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
	/// Reverse a string
	Reverse {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
	},

	/// Convert a string to UPPER CASE
	Upper {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
	},
	/// Convert a string to lower case
	Lower {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
	},
	/// Convert a string to Title Case
	Title {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
	},
	/// Convert a string to aLtErNaTiNg CaSe
	Alternating {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
	},
	/// Convert a string to camelCase
	Camel {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
	},
	/// Convert a string to PascalCase
	Pascal {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
	},
	/// Convert a string to snake_case
	Snake {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
	},
	/// Convert a string to SCREAMING_SNAKE_CASE
	Screaming {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
	},
	/// Convert a string to SCREAMING_SNAKE_CASE
	ScreamingSnake {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
	},
	/// Convert a string to kebab-case
	Kebab {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
	},

	/// Count newlines in a string
	CountLines {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
	},
	/// Count total characters or the occurrences of a given character
	CountChars {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
		character: Option<char>,
	},
	/// Count words in a string or the occurrences of a given word
	CountWords {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
		word: Option<String>,
	},
}

fn main() {
	let args = Cli::parse();

	if let Some(command) = args.command {
		match command {
			Commands::Reverse { text } => {
				println!("{}", text.chars().rev().collect::<String>());
			}

			Commands::Upper { text } => {
				println!("{}", text.to_case(Case::Upper));
			}
			Commands::Lower { text } => {
				println!("{}", text.to_case(Case::Lower));
			}
			Commands::Title { text } => {
				println!("{}", text.to_case(Case::Title));
			}
			Commands::Alternating { text } => {
				println!("{}", text.to_case(Case::Alternating));
			}
			Commands::Camel { text } => {
				println!("{}", text.to_case(Case::Camel));
			}
			Commands::Pascal { text } => {
				println!("{}", text.to_case(Case::Pascal));
			}
			Commands::Snake { text } => {
				println!("{}", text.to_case(Case::Snake));
			}
			Commands::Screaming { text } | Commands::ScreamingSnake { text } => {
				println!("{}", text.to_case(Case::ScreamingSnake));
			}
			Commands::Kebab { text } => {
				println!("{}", text.to_case(Case::Kebab));
			}

			Commands::CountLines { text } => {
				println!("{}", text.lines().count());
			}
			Commands::CountChars { text, character } => {
				if let Some(character) = character {
					println!("{}", text.chars().filter(|c| *c == character).count());
				} else {
					println!("{}", text.chars().count());
				}
			}
			Commands::CountWords { text, word } => {
				let words = text.unicode_words().collect::<Vec<&str>>();
				if let Some(word) = word {
					println!(
						"{}",
						words
							.iter()
							.filter(|&w| *w.to_lowercase() == word.to_lowercase())
							.count()
					);
				} else {
					println!("{}", words.len());
				}
			}
		}
	}
}
