mod cli;
mod cmd;
mod epub;

use crate::cli::Arguments;
use epub::Article;

#[macro_use]
extern crate error_chain;

fn main() {
    let args = Arguments::cli();
    println!("{:#?}", args);

    let url = args.url.unwrap_or(args.rustpub_test_url);

    let res = Article::epub_from_url(url);
    println!("{:?}", res);
}
