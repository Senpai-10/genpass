use crate::consts::{LOWERCASE_CHARACTERS_SET, NUMBERS_SET, SYMBOLS_SET, UPPERCASE_CHARACTERS_SET};
use crate::Args;

pub fn build_charset(args: &Args) -> String {
    let mut charset = String::new();

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

    return charset;
}
