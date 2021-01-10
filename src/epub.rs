use std::io;
use std::fs;

extern crate tera;

extern crate url;
extern crate epub_builder;
extern crate serde;
extern crate serde_json;

use tempfile::Builder;
use url::Url;
use serde::Deserialize;
use epub_builder::{EpubBuilder, ZipLibrary, EpubContent};

use crate::parse::{RustpubParser, readability_rs_parse, readabilipy_parse};
use crate::error::errors::*;
use crate::img::{extract_image_urls, download_images};

#[derive(Deserialize, Debug, Clone)]
pub struct Document {
    pub title: Option<String>,
    pub byline: Option<String>,  // author
    pub date: Option<String>,
    pub content: Option<String>,
    pub plain_text: Option<String>,
}

impl Document {
    pub async fn epub_from_url(target: String, output: String, parser: RustpubParser) -> Result<()> {
        // Parse target URL
        let target_url = Url::parse(&target);

        // Check target URL validity
        match target_url {
            Ok(url) => { println!("{}", url) },
            Err(e) => {
                println!("Error {}, return.", e);
                // return Err(InvalidUrl)
                return Ok(())  // TODO: Implement Error InvalidURL
            }
        };  // match target_url

        let tmp_dir = Builder::new().prefix("rustpub_").tempdir()?;  // Make temp dir
        let tmp_dir_path = tmp_dir.into_path();  // Persist the tempdir and return PathBuf
        let document: Document;

        match parser {
            RustpubParser::ReadabilityRs => {
                document = readability_rs_parse(&target)?;
            },
            RustpubParser::ReadabilityJs | RustpubParser::ReadabiliPy => {
                let json_file = readabilipy_parse(&target, parser, tmp_dir_path.clone()).await?;
                document = serde_json::from_reader(json_file).expect("error reading json");
            }
        };

        // Get absolute image urls
        let image_urls = extract_image_urls(target.clone(), document.clone().content);
        // Download images and store metadata
        let image_metas = download_images(image_urls, target.clone(), tmp_dir_path.clone()).await?;

        // TODO: Image URL correction in content

        // Build epub
        let mut epub: Vec<u8> = vec!();
        let epub_filename = format!("{}.epub", output);
        let mut epub_dest = fs::File::create(epub_filename)?;  // TODO: use sluggified title if None

        let epub_title = document.title.unwrap_or("Unknown".into());
        let epub_author = document.byline.unwrap_or("Unknown".into());

        let css_file = fs::File::open(&"assets/stylesheet.css")?;

        // Rendering content
        let mut tera = match tera::Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        tera.autoescape_on(vec![]);  // Don't escape context values!

        let mut context = tera::Context::new();
        context.insert("title", &epub_title);
        context.insert("author", &epub_author);
        context.insert("content", &document.content.unwrap_or("Unknown".into()));

        let epub_content = tera.render("introduction.html", &context)?;

        // Building
        let mut builder = EpubBuilder::new(ZipLibrary::new()?)?;

        builder.metadata("author", epub_author)?;
        builder.metadata("title", epub_title.clone())?;
        builder.stylesheet(css_file)?;
        builder.add_content(EpubContent::new("document.xhtml", epub_content.as_bytes()))?;

        for imeta in image_metas {
            let ext = format!("image/{}", imeta.extension.unwrap());
            let img = fs::File::open(&imeta.local_path.unwrap())?;
            builder.add_resource(imeta.filename.unwrap(), &img, ext)?;
        };

        builder.inline_toc(); // Index in document
        builder.generate(&mut epub)?;

        io::copy(&mut &epub[..], &mut epub_dest)
            .expect("Failed to copy epub file to destination");

        // Delete the temporary directory ourselves.
        fs::remove_dir_all(tmp_dir_path)?;

        Ok(())
    }
}  // Document
