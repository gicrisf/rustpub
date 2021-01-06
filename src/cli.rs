use clap::{crate_version, Arg};
use std::env;

#[derive(Debug)]
pub struct Arguments {
    pub url: Option<String>,
    pub output: String,
    pub rustpub_test_url: String,
    pub verbose: bool,
}

impl Arguments {
    // rustpub -u https://... -o nomelibro
    pub fn cli() -> Arguments {
        let env_key = env::var("RUSTPUB_TEST_URL");

        let matches = clap::App::new("rustpub")
            .version(crate_version!())
            .author("Giovanni Crisalfi <giovanni.crisalfi@gmail.com>")
            .about("Download your favorite webpages as epub.")
            .arg(
                Arg::with_name("url")
                    .short("u")
                    .long("url")
                    .help("The webpage URL")
                    .required(true)
                    .takes_value(true)
                    .conflicts_with("test_url"),
            )
            .arg(
                Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .help("The generated file")
                    .required(false)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("test_url")
                    .short("t")
                    .help("URL for testing")
                    .takes_value(false)
                    .required(env_key.is_err()),
            )
            .arg(
                Arg::with_name("verbose")
                    .short("v")
                    .long("verbose")
                    .help("Prints extra information, used for debugging"),
            )
            .get_matches();

        Arguments {
            url: matches.value_of("url").map(|s| s.to_string()),

            output: match matches.value_of("output") {
                Some(d) => d.to_string(),
                None => "ebook".to_string(),
            },

            rustpub_test_url: match matches.value_of("test_url") {
                Some(a) => a.to_string(),
                None => {
                    if !env_key.is_err() {
                        env_key.unwrap().to_string()
                    } else {
                        "".to_string()  // Put default test here
                    }
                }
            },

            verbose: matches.is_present("verbose"),
        }  // Arguments
    }
}
