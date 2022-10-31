mod args;
mod build_charset;
mod consts;

use args::Args;
use build_charset::build_charset;
use clap::Parser;
use consts::{MAX_LENGTH, MIN_LENGTH};
use random_string::generate;

fn main() {
    let args = Args::parse();

    match args.command {
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
