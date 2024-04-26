use clap::{Parser, Subcommand};
use clap_stdin::MaybeStdin;

use eyre::{Result, WrapErr};

use base64::{engine::general_purpose, Engine as _};
use convert_case::{Case, Casing};
use rand::seq::SliceRandom;
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

	/// Reverse the lines of a string
	ReverseLines {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
	},
	/// Shuffle the lines of a string
	ShuffleLines {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
	},

	/// Sort the lines of a string
	SortLines {
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

	/// Replace all occurrences of a search string with a replace string in a string
	Replace {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
		#[clap(default_value = "")]
		find: String,
		#[clap(default_value = "")]
		replace: String,
	},

	/// Encode a string in base64
	Base64Encode {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
	},
	/// Decode a base64 string
	Base64Decode {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
	},
}

fn main() -> Result<()> {
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

			Commands::ReverseLines { text } => {
				println!("{}", text.lines().rev().collect::<Vec<_>>().join("\n"))
			}
			Commands::ShuffleLines { text } => {
				let mut lines: Vec<_> = text.lines().collect();
				lines.shuffle(&mut rand::thread_rng());
				println!("{}", lines.join("\n"));
			}
			Commands::SortLines { text } => {
				let mut lines: Vec<_> = text.lines().collect();
				lines.sort();
				println!("{}", lines.join("\n"));
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

			Commands::Replace {
				text,
				find,
				replace,
			} => {
				println!("{}", text.replace(&find, &replace));
			}

			Commands::Base64Encode { text } => {
				println!("{}", general_purpose::STANDARD.encode(text.as_bytes()));
			}
			Commands::Base64Decode { text } => {
				println!(
					"{}",
					String::from_utf8_lossy(
						&general_purpose::STANDARD
							.decode(text.as_bytes())
							.with_context(|| "Failed to decode base64 string")?
					)
				);
			}
		}
	}

	Ok(())
}
