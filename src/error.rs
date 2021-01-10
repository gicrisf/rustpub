pub mod errors {
    error_chain! {
         foreign_links {
             Io(std::io::Error);
             HttpRequest(reqwest::Error);
             EpubBuilding(epub_builder::Error);
             ImageReading(image::ImageError);
             Tera(tera::Error);
             Readability(readability::error::Error);
             Anyhow(anyhow::Error);
         }
    }
}
