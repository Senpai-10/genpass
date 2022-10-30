use clap::Parser;
use crate::consts::MIN_LENGTH;

/// password generator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Length of password generated
    #[arg(short, long, default_value_t = MIN_LENGTH)]
    pub length: usize,

    /// Number of password that will be generated
    #[arg(short, long, default_value_t = 1)]
    pub number_of_passwords: usize,

    /// Verbose output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Before password
    #[arg(long)]
    pub prefix: Option<String>,

    /// After password
    #[arg(long)]
    pub suffix: Option<String>,

    /// List of characters to exclude
    #[arg(short, long)]
    pub exclude: Option<String>,

    /// Custom charset to generate passwords from
    #[arg(long)]
    pub charset: Option<String>,

    /// Append custom charset to charset
    #[arg(long)]
    pub charset_append: Option<String>,

    // TODO retry with MAX_RETRYS const
    #[arg(short, long, default_value_t = false)]
    pub allow_duplicate_passwords: bool,

    /// Remove lowercase characters from default charset
    #[arg(short = 'L', long, default_value_t = false)]
    pub no_lowercase_characters: bool,

    /// Remove uppercase characters from default charset
    #[arg(short = 'U', long, default_value_t = false)]
    pub no_uppercase_characters: bool,

    /// Remove number from default charset
    #[arg(short = 'N', long, default_value_t = false)]
    pub no_numbers: bool,

    /// Remove symbols from default charset
    #[arg(short = 'S', long, default_value_t = false)]
    pub no_symbols: bool,
}
