use clap::{crate_version, Arg};
use std::env;

#[derive(Debug)]
pub struct Arguments {
    pub url: Option<String>,
    pub output: String,
    pub test_url: String,
    pub verbose: bool,
    pub parser: Option<String>,
    pub bw_images: bool,
    pub image_max_size: Option<u32>,
}

impl Arguments {
    pub fn cli() -> Arguments {
        let env_key = env::var("RUSTPUB_TEST_URL");
        let env_max = env::var("RUSTPUB_MAX_IMG");

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
                Arg::with_name("parser")
                    .short("p")
                    .long("parser")
                    .takes_value(true)
                    .help("Select parser that will sanitize the webpage."),
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
                    .required(false),
            )
            .arg(
                Arg::with_name("image_max_size")
                    .long("max")
                    .help("Set Image's MAX size (preserve aspect ratio)")
                    .takes_value(true)
                    .required(false),
            )
            .arg(
                Arg::with_name("bw_images")
                    .long("bw")
                    .help("Save space on disk converting every image to a B&W equivalent with no Alpha channel")
                    .takes_value(false)
                    .required(false),
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

            image_max_size: match matches.value_of("image_max_size") {
                Some(d) => Some(d.to_string().parse::<u32>().unwrap()),
                None => {
                    if !env_max.is_err() {
                        Some(env_max.unwrap().to_string().parse::<u32>().unwrap())
                    } else {
                        None
                    }
                },
            },

            bw_images: matches.is_present("bw_images"),

            parser: matches.value_of("parser").map(|s| s.to_string()),

            test_url: match matches.value_of("test_url") {
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
