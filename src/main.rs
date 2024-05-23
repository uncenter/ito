use clap::{Parser, Subcommand};
use clap_stdin::MaybeStdin;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
	prelude::*,
	symbols::border,
	widgets::{block::*, *},
};
use std::io;

use eyre::Result;

mod tui;
mod lib {
	pub mod base64;
	pub mod case;
	pub mod count;
	pub mod lines;
	pub mod misc;
}

use lib::*;

#[derive(Debug, Default)]
pub struct App {
	counter: u8,
	exit: bool,
}

impl App {
	pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
		while !self.exit {
			terminal.draw(|frame| self.render_frame(frame))?;
			self.handle_events()?;
		}
		Ok(())
	}

	fn render_frame(&self, frame: &mut Frame) {
		frame.render_widget(self, frame.size());
	}

	fn handle_events(&mut self) -> io::Result<()> {
		match event::read()? {
			Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
				self.handle_key_event(key_event)
			}
			_ => {}
		};
		Ok(())
	}

	fn handle_key_event(&mut self, key_event: KeyEvent) {
		match key_event.code {
			KeyCode::Char('q') => self.exit(),
			KeyCode::Left => self.decrement_counter(),
			KeyCode::Right => self.increment_counter(),
			_ => {}
		}
	}

	fn exit(&mut self) {
		self.exit = true;
	}

	fn increment_counter(&mut self) {
		self.counter += 1;
	}

	fn decrement_counter(&mut self) {
		self.counter -= 1;
	}
}

impl Widget for &App {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let title = Title::from(" sttr ".bold());
		let instructions = Title::from(Line::from(vec![
			" Decrement ".into(),
			"<Left>".blue().bold(),
			" Increment ".into(),
			"<Right>".blue().bold(),
			" Quit ".into(),
			"<Q> ".blue().bold(),
		]));
		let block = Block::default()
			.title(title.alignment(Alignment::Center))
			.title(
				instructions
					.alignment(Alignment::Center)
					.position(Position::Bottom),
			)
			.borders(Borders::ALL)
			.border_set(border::THICK);

		let counter_text = Text::from(vec![Line::from(vec![
			"Value: ".into(),
			self.counter.to_string().yellow(),
		])]);

		Paragraph::new(counter_text)
			.centered()
			.block(block)
			.render(area, buf);
	}
}

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

	/// Join strings with a given joiner
	Join {
		text: Vec<String>,
		#[clap(default_value = "\n", last(true))]
		joiner: String,
	},
	/// Reverse a string
	Reverse {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
	},
	/// Repeat a string x times
	Repeat {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
		#[clap(default_value = "1")]
		count: usize,
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
	/// Trim trailing whitespace and newlines from a string
	Trim {
		#[clap(default_value = "-")]
		text: MaybeStdin<String>,
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

				Commands::Join { text, joiner } => misc::join(text, joiner),
				Commands::Reverse { text } => misc::reverse(text),
				Commands::Repeat { text, count } => misc::repeat(text, count),
				Commands::Replace {
					text,
					find,
					replace,
				} => misc::replace(text, find, replace),
				Commands::Trim { text } => misc::trim(text),
			}
		);
	} else {
		let mut terminal = tui::init()?;
		let app_result = App::default().run(&mut terminal);
		tui::restore()?;
		app_result?
	}

	Ok(())
}
