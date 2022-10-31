use crate::consts::MIN_LENGTH;
use clap::{Parser, Subcommand};

/// password generator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Passphrase {
        /// Number of words in the passphrase
        #[arg(short, long, default_value_t = 3)]
        number_of_words: usize,

        /// Separator between words
        #[arg(short, long, default_value_t = String::from("-"))]
        word_separator: String,

        /// Capitalize the first letter of every words
        #[arg(short, long, default_value_t = false)]
        capitalize: bool,

        /// Add single diget number at the end of a word
        #[arg(short, long, default_value_t = false)]
        include_number: bool,

        /// Number of passphrases to generate
        #[arg(short, long, default_value_t = 1)]
        number_of_passphrases: usize,
    },

    Password {
        /// Length of password generated
        #[arg(short, long, default_value_t = MIN_LENGTH)]
        length: usize,

        /// Number of password that will be generated
        #[arg(short, long, default_value_t = 1)]
        number_of_passwords: usize,

        /// Verbose output
        #[arg(short, long, default_value_t = false)]
        verbose: bool,

        /// Before password
        #[arg(long)]
        prefix: Option<String>,

        /// After password
        #[arg(long)]
        suffix: Option<String>,

        /// List of characters to exclude
        #[arg(short, long)]
        exclude: Option<String>,

        /// Custom charset to generate passwords from
        #[arg(long)]
        charset: Option<String>,

        /// Append custom charset to charset
        #[arg(long)]
        charset_append: Option<String>,

        // TODO retry with MAX_RETRYS const
        #[arg(short, long, default_value_t = false)]
        allow_duplicate_passwords: bool,

        /// Remove lowercase characters from default charset
        #[arg(short = 'L', long, default_value_t = false)]
        no_lowercase_characters: bool,

        /// Remove uppercase characters from default charset
        #[arg(short = 'U', long, default_value_t = false)]
        no_uppercase_characters: bool,

        /// Remove number from default charset
        #[arg(short = 'N', long, default_value_t = false)]
        no_numbers: bool,

        /// Remove symbols from default charset
        #[arg(short = 'S', long, default_value_t = false)]
        no_symbols: bool,
    },
}
