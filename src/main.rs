use rustpub::cli::Arguments;
use rustpub::epub::Document;
use rustpub::parse::RustpubParser;

#[tokio::main]
async fn main() {
    let args = Arguments::cli();
    let url = args.url.unwrap_or(args.rustpub_test_url);

    let parser = args.parser.unwrap_or("".into());

    let parser = match &parser[..] {
        "py" => RustpubParser::ReadabiliPy,
        "js" => RustpubParser::ReadabilityJs,
        "rs" => RustpubParser::ReadabilityRs,
        _ => RustpubParser::ReadabilityRs,
    };

    let _res = Document::epub_from_url(url, args.output, parser).await;
}
