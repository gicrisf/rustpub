use rustpub::cli::Arguments;
use rustpub::epub::Document;
use rustpub::parse::ParserKind;

#[tokio::main]
async fn main() {
    let args = Arguments::cli();

    // Get vars from args
    let target = args.url.unwrap_or(args.test_url);
    let parser = args.parser.unwrap_or("".into());
    let image_max_size = args.image_max_size.unwrap_or(1000);
    let bw_images = args.bw_images;

    let parser = match &parser[..] {
        "py" => ParserKind::ReadabiliPy,
        "js" => ParserKind::ReadabilityJs,
        "rs" => ParserKind::ReadabilityRs,
        _ => ParserKind::ReadabilityRs,
    };

    let output: String = args.output;

    let _ = Document::epub_from_url(target, output, parser, bw_images, image_max_size).await;
}
