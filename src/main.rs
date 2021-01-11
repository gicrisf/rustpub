use rustpub::cli::Arguments;
use rustpub::epub::Document;
use rustpub::parse::ParserKind;

#[tokio::main]
async fn main() {
    let args = Arguments::cli();
    let _res = Document::epub_from_url(args.clone()).await;
}
