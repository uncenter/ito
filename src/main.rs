use clap::{Parser, Subcommand};
use clap_stdin::MaybeStdin;
use eyre::Result;

mod lib {
    pub mod base64;
    pub mod case;
    pub mod count;
    pub mod lines;
    pub mod misc;
    pub mod hash;
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
    /// Generate bcrypt hash
    Bcrypt {
        #[clap(default_value = "-")]
        text: MaybeStdin<String>,
    },
    /// Generate MD5 hash
    Md5 {
        #[clap(default_value = "-")]
        text: MaybeStdin<String>,
    },
    /// Generate SHA1 hash
    Sha1 {
        #[clap(default_value = "-")]
        text: MaybeStdin<String>,
    },
    /// Generate SHA256 hash
    Sha256 {
        #[clap(default_value = "-")]
        text: MaybeStdin<String>,
    },
    /// Generate SHA512 hash
    Sha512 {
        #[clap(default_value = "-")]
        text: MaybeStdin<String>,
    },
    /// Encode a string in base64url
    Base64urlEncode {
        #[clap(default_value = "-")]
        text: MaybeStdin<String>,
    },
    /// Decode a base64url string
    Base64urlDecode {
        #[clap(default_value = "-")]
        text: MaybeStdin<String>,
    },
    /// URL encode a string
    UrlEncode {
        #[clap(default_value = "-")]
        text: MaybeStdin<String>,
    },
    /// URL decode a string
    UrlDecode {
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
                Commands::Screaming { text } | Commands::ScreamingSnake { text } => case::screaming(text),
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
                Commands::Replace { text, find, replace } => misc::replace(text, find, replace),
                Commands::Trim { text } => misc::trim(text),
                Commands::Bcrypt { text } => hash::bcrypt(text),
                Commands::Md5 { text } => hash::md5(text),
                Commands::Sha1 { text } => hash::sha1(text),
                Commands::Sha256 { text } => hash::sha256(text),
                Commands::Sha512 { text } => hash::sha512(text),
                Commands::Base64urlEncode { text } => base64::base64url_encode(text),
                Commands::Base64urlDecode { text } => base64::base64url_decode(text).unwrap(),
                Commands::UrlEncode { text } => base64::url_encode(text),
                Commands::UrlDecode { text } => base64::url_decode(text).unwrap(),
            }
        );
    }
    Ok(())
}