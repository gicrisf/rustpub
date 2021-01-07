mod cli;
mod cmd;
mod epub;
mod parser;

use crate::cli::Arguments;
use epub::Document;

#[macro_use]
extern crate error_chain;

pub enum RustpubParser {
    BeautifulSoup,
    Mozilla,
    Kuchiki,
}

fn main() {
    let args = Arguments::cli();
    let url = args.url.unwrap_or(args.rustpub_test_url);

    let parser = args.parser.unwrap_or("".into());

    let parser = match &parser[..] {
        "py" => RustpubParser::BeautifulSoup,
        "js" => RustpubParser::Mozilla,
        "rs" => RustpubParser::Kuchiki,
        _ => RustpubParser::Mozilla,
    };

    let _res = Document::epub_from_url(url, args.output, parser);
}
