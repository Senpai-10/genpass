mod args;
mod consts;

use args::Args;
use clap::Parser;
use consts::{
    LOWERCASE_CHARACTERS_SET, MAX_LENGTH, MIN_LENGTH, NUMBERS_SET, SYMBOLS_SET,
    UPPERCASE_CHARACTERS_SET,
};
use random_string::generate;

fn main() {
    let mut args = Args::parse();
    let mut charset = String::new();

    // There is not need for a password that's more than 255 in length
    if args.length > MAX_LENGTH {
        args.length = MAX_LENGTH;
    } else if args.length < MIN_LENGTH {
        args.length = MIN_LENGTH
    }

    if args.no_lowercase_characters == false {
        charset.push_str(LOWERCASE_CHARACTERS_SET)
    }

    if args.no_uppercase_characters == false {
        charset.push_str(UPPERCASE_CHARACTERS_SET)
    }

    if args.no_numbers == false {
        charset.push_str(NUMBERS_SET)
    }

    if args.no_symbols == false {
        charset.push_str(SYMBOLS_SET)
    }

    if let Some(args_charset) = &args.charset {
        charset = args_charset.to_string();
    }

    if let Some(args_charset_append) = &args.charset_append {
        charset.push_str(args_charset_append)
    }

    if let Some(exclude) = &args.exclude {
        let chars_vec: Vec<char> = exclude.chars().collect();

        charset = charset.replace(&chars_vec[..], "");
    }

    // This check is here, Because you can remove all chars in charset with --no options
    if charset.is_empty() {
        charset.push_str(SYMBOLS_SET);
        charset.push_str(NUMBERS_SET);
        charset.push_str(LOWERCASE_CHARACTERS_SET);
        charset.push_str(UPPERCASE_CHARACTERS_SET);
    }

    let length = args.length;

    for _ in 0..args.number_of_passwords {
        let generated_password = generate(length, &charset);

        println!("{}", generated_password);
    }
}
