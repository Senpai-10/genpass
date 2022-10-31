mod args;
mod build_charset;
mod consts;

use args::Args;
use build_charset::build_charset;
use clap::Parser;
use consts::{MAX_LENGTH, MIN_LENGTH, WORDS_LIST};
use rand::seq::SliceRandom;
use random_string::generate;

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        args::Commands::Passphrase {
            number_of_words,
            word_separator,
            capitalize,
            number_of_passphrases,
        } => {
            for _ in 0..number_of_passphrases {
                let mut passphrase = String::new();

                for i in 0..number_of_words {
                    let word = WORDS_LIST.choose(&mut rand::thread_rng()).unwrap();

                    if capitalize {
                        passphrase.push_str(&uppercase_first_letter(word));
                    } else {
                        passphrase.push_str(word);
                    }

                    if i != (number_of_words - 1) {
                        passphrase.push(word_separator);
                    }
                }

                println!("{}", passphrase);
            }
        }
        args::Commands::Password {
            mut length,
            prefix,
            suffix,
            verbose,
            exclude,
            charset,
            no_numbers,
            no_symbols,
            charset_append,
            number_of_passwords,
            no_lowercase_characters,
            no_uppercase_characters,
            allow_duplicate_passwords,
        } => {
            if length > MAX_LENGTH {
                length = MAX_LENGTH;
            } else if length < MIN_LENGTH {
                length = MIN_LENGTH
            }

            let charset = build_charset(
                no_lowercase_characters,
                no_uppercase_characters,
                no_numbers,
                no_symbols,
                charset,
                charset_append,
                exclude,
            );

            let length = length;
            let mut passwords: Vec<String> = Vec::new();

            for _ in 0..number_of_passwords {
                let mut generated_password = generate(length, &charset);

                if let Some(prefix) = &prefix {
                    let mut tmp_string = String::new();

                    tmp_string.push_str(prefix);
                    tmp_string.push_str(&generated_password);

                    generated_password = tmp_string;
                }

                if let Some(suffix) = &suffix {
                    generated_password.push_str(suffix);
                }

                if allow_duplicate_passwords == false {
                    if passwords.contains(&generated_password) {
                        continue;
                    }
                }

                println!("{}", generated_password);

                if allow_duplicate_passwords == false {
                    passwords.push(generated_password);
                }
            }

            if verbose {
                println!();
                println!("{} passwords generated!", length);
            }
        }
    }
}
