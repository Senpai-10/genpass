mod args;
mod build_charset;
mod consts;

use args::Args;
use build_charset::build_charset;
use clap::Parser;
use consts::{MAX_LENGTH, MIN_LENGTH};
use random_string::generate;

fn main() {
    let mut args = Args::parse();

    if args.length > MAX_LENGTH {
        args.length = MAX_LENGTH;
    } else if args.length < MIN_LENGTH {
        args.length = MIN_LENGTH
    }

    let charset = build_charset(&args);

    let length = args.length;

    for _ in 0..args.number_of_passwords {
        let mut generated_password = generate(length, &charset);

        if let Some(prefix) = &args.prefix {
            let mut tmp_string = String::new();

            tmp_string.push_str(prefix);
            tmp_string.push_str(&generated_password);

            generated_password = tmp_string;
        }

        if let Some(suffix) = &args.suffix {
            generated_password.push_str(suffix);
        }

        println!("{}", generated_password);
    }
}
