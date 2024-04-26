use clap::{Parser, Subcommand};
use clap_stdin::MaybeStdin;

use eyre::Result;

mod lib {
	pub mod base64;
	pub mod case;
	pub mod count;
	pub mod lines;
	pub mod misc;
}

use lib::*;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
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

	/// Reverse a string
	Reverse {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
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
}

fn main() -> Result<()> {
	let args = Cli::parse();

	if let Some(command) = args.command {
		println!(
			"{}",
			match command {
				Commands::Base64Encode { text } => base64::encode(text),
				Commands::Base64Decode { text } => base64::decode(text).unwrap(),

				Commands::Upper { text } => case::upper(text),
				Commands::Lower { text } => case::lower(text),
				Commands::Title { text } => case::title(text),
				Commands::Alternating { text } => case::alternating(text),
				Commands::Camel { text } => case::camel(text),
				Commands::Pascal { text } => case::pascal(text),
				Commands::Snake { text } => case::snake(text),
				Commands::Screaming { text } | Commands::ScreamingSnake { text } => {
					case::screaming(text)
				}
				Commands::Kebab { text } => case::kebab(text),

				Commands::CountLines { text } => count::count_lines(text),
				Commands::CountChars { text, character } => count::count_chars(text, character),
				Commands::CountWords { text, word } => count::count_words(text, word),

				Commands::ReverseLines { text } => lines::reverse_lines(text),
				Commands::ShuffleLines { text } => lines::shuffle_lines(text),
				Commands::SortLines { text } => lines::sort_lines(text),

				Commands::Reverse { text } => misc::reverse(text),
				Commands::Replace {
					text,
					find,
					replace,
				} => misc::replace(text, find, replace),
			}
		);
	}

	Ok(())
}
