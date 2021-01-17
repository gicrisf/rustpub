extern crate readability;

use readability::extractor;

use crate::error::errors::*;
use crate::epub::Document;
use crate::cmd::ReadabiliPyCmd;

pub struct MyScraper {
    fragment: scraper::html::Html
}

impl MyScraper {
    pub fn new(html: &str) -> Self {
        let fragment = scraper::Html::parse_fragment(&html);
        Self { fragment }
    } // new

    fn extract_from_meta_string(&self, meta_string: &str) -> String {
        let extracted = match scraper::Selector::parse(&meta_string) {
            Ok(selection) => {
                let mut content = format!("Unknown {}", meta_string);

                for element in self.fragment.select(&selection) {
                    content = element.value().attr("content").unwrap_or("Unknown").into();
                };

                content
            },
            Err(e) => {
                println!("{:?}", e);
                "Unknown".into()
            }
        };

        extracted
    }

    pub fn extract_meta_name(&self, name: &str) -> String {
        let meta_string = format!("meta[name=\"{}\"]", name);
        self.extract_from_meta_string(&meta_string)
    }  // extract_meta_property

    pub fn extract_meta_property(&self, property: &str) -> String {
        let meta_string = format!("meta[property=\"{}\"]", property);
        self.extract_from_meta_string(&meta_string)
    }  // extract_meta_property
}

pub enum ParserKind {
    ReadabiliPy,
    ReadabilityJs,
    ReadabilityRs,
}

pub struct Parser {}

impl Parser {
    pub fn default(target: &str) -> anyhow::Result<Document> {
        let product = extractor::scrape(target)?;
        // println!("{}", product.text);

        let document = Document {
            title: Some(product.title),
            byline: Some("Unknown".into()),
            date: Some("Unknown".into()),
            content: Some(product.content),
            plain_text: Some(product.text),
        };

        Ok(document)
    }

    pub async fn readabilipy(target: &str, parser: ParserKind, tmp_dir_path: std::path::PathBuf) -> Result<std::fs::File> {
        // Make HTTP request for target file
        let response = reqwest::get(target.clone()).await?;

        // Choosing filename
        let filename = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        // println!("file to download: '{:?}'", filename);

        // Locate destination
        let local_abs_path = tmp_dir_path.clone().join(filename);
        let local_abs_pathstr = local_abs_path.clone().into_os_string().into_string().unwrap();
        let mut destination = std::fs::File::create(local_abs_path.clone())?;
        let html_string = response.text().await?;
        std::io::copy(&mut html_string.as_bytes(), &mut destination).expect("Failed to copy HTML file");

        // Generate json file with ReadabiliPy
        let outfile_path = tmp_dir_path.join("document.json");  // TODO: use fname
        let outfile_path_string = outfile_path.clone().into_os_string().into_string().unwrap();

        ReadabiliPyCmd::json_from_file(
            parser,
            local_abs_pathstr,
            outfile_path_string
        );

        // Read Json, deserialize and print Rust data structure.
        let json_file = std::fs::File::open(outfile_path).expect("file not found");
        Ok(json_file)
    }
}
