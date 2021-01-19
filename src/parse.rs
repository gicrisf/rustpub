extern crate readability;

use readability::extractor;

use crate::error::errors::*;
use crate::epub::Document;
use crate::cmd::ReadabiliPyCmd;

pub struct MetaScraper {
    fragment: scraper::html::Html
}

impl MetaScraper {
    pub fn new(html: &str) -> Self {
        let fragment = scraper::Html::parse_fragment(&html);
        Self { fragment }
    } // new

    fn extract_from_meta_string(&self, meta_string: &str) -> Option<&str> {
        let extracted = match scraper::Selector::parse(&meta_string) {
            Ok(selection) => {
                let mut content = None;

                for element in self.fragment.select(&selection) {
                    content = element.value().attr("content");  // &str
                };

                content
            },
            Err(e) => {
                println!("{:?}", e);
                None
            }
        };

        extracted
    }

    fn extract_meta_name(&self, name: &str) -> Option<&str> {
        let meta_string = format!("meta[name=\"{}\"]", name);
        self.extract_from_meta_string(&meta_string)
    }  // extract_meta_name

    fn extract_meta_property(&self, property: &str) -> Option<&str> {
        let meta_string = format!("meta[property=\"{}\"]", property);
        self.extract_from_meta_string(&meta_string)
    }  // extract_meta_property

    fn extract_meta_itemprop(&self, itemprop: &str) -> Option<&str> {
        let meta_string = format!("meta[itemprop=\"{}\"]", itemprop);
        self.extract_from_meta_string(&meta_string)
    }

    // Public wrapper functions
    pub fn extract_date(&self) -> Option<String> {
        let date = match self.extract_meta_name("article:published_time") {
            Some(d) => Some(d.into()),
            None => {
                match self.extract_meta_property("og:updated_time") {
                    Some(og_d) => Some(og_d.into()),
                    None => {
                        match self.extract_meta_itemprop("datePublished") {
                            Some(schema_d) => Some(schema_d.into()),
                            None => None
                        }  // match schema_d
                    }  // No og updated_time
                }  // match og updated_time
            }  // No article:published_time
        };

        date  // match article:published_time
    }

    pub fn extract_title(&self) -> Option<String> {
        let title = match self.extract_meta_property("og:title") {
            Some(t) => Some(t.into()),
            None => {
                match self.extract_meta_name("twitter:title") {
                    Some(twitter_t) => Some(twitter_t.into()),
                    None => None
                }  // match twitter
            }  // No og title
        };  // match og title

        title
    }  // extract title

    pub fn extract_author(&self) -> Option<String> {
        let author = match self.extract_meta_property("og:author") {
            Some(a) => Some(a.into()),
            None => None
        };

        author
    }  // extract_author
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
