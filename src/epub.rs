use std::io;
use std::fs;

extern crate tera;
// extern crate image;
extern crate soup;
extern crate url;
extern crate epub_builder;
extern crate serde;
extern crate serde_json;
extern crate readability;

use readability::extractor;
use tempfile::Builder;
// use image::io::Reader as ImageReader;
use soup::prelude::*;
use url::{Url, ParseError};
use epub_builder::{EpubBuilder, ZipLibrary, EpubContent};
use serde::Deserialize;
use crate::cmd::ReadabiliPyCmd;
use crate::RustpubParser;

mod errors {
    error_chain! {
         foreign_links {
             Io(std::io::Error);
             HttpRequest(reqwest::Error);
             EpubBuilding(epub_builder::Error);
             ImageReading(image::ImageError);
             Tera(tera::Error);
             Readability(readability::error::Error);
         }
    }
}

use errors::*;

struct ImgMeta {
    url: Option<Url>,
    filename: Option<String>,
    extension: Option<String>,
    local_path: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Document {
    title: Option<String>,
    byline: Option<String>,  // author
    date: Option<String>,
    content: Option<String>,
    plain_text: Option<String>,
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
                let product = extractor::scrape(&target)?;
                // println!("{}", product.text);

                document = Document {
                    title: Some(product.title),
                    byline: Some("Author".into()),  // TODO
                    date: Some("Date".into()),  // TODO
                    content: Some(product.content),
                    plain_text: Some(product.text),
                }
            },
            RustpubParser::ReadabilityJs | RustpubParser::ReadabiliPy => {
                // Make HTTP request for target file
                let response = reqwest::get(&target.clone()).await?;

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
                let mut destination = fs::File::create(local_abs_path.clone())?;
                let html_string = response.text().await?;
                io::copy(&mut html_string.as_bytes(), &mut destination).expect("Failed to copy HTML file");

                // Generate json file with ReadabiliPy
                let outfile_path = tmp_dir_path.join("document.json");  // TODO: use fname
                let outfile_path_string = outfile_path.clone().into_os_string().into_string().unwrap();

                ReadabiliPyCmd::json_from_file(
                    parser,
                    local_abs_pathstr,
                    outfile_path_string
                );

                // Read Json, deserialize and print Rust data structure.
                let json_file = fs::File::open(outfile_path).expect("file not found");
                document = serde_json::from_reader(json_file).expect("error reading json");
            }
        };

        // Get absolute image urls
        let image_urls = Document::extract_image_urls(target.clone(), document.clone().content);

        // Download images
        let mut image_metas = Vec::new();

        for url in image_urls {
            // Make HTTP request for target file
            let response = reqwest::get(target.as_str()).await?;

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

            let local_abs_pathstr = local_abs_path.clone().into_os_string().into_string().expect("local abs path string error");
            let ext = local_abs_path.extension().unwrap().to_os_string().into_string().unwrap_or("jpg".into());
            let point_ext = format!(".{}", ext.clone());
            let filename = filename.replace(&point_ext, "");
            let mut destination = fs::File::create(local_abs_path.clone())?;

            let mut bytes = &response.bytes().await?[..];

            io::copy(&mut bytes, &mut destination).expect("Failed to copy image to dest.");

            let meta = ImgMeta {
                url: Some(url),
                filename: Some(filename),
                extension: Some(ext),
                local_path: Some(local_abs_pathstr),
            };

            image_metas.push(meta);
        }

        // Image URL correction in content

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

    fn extract_image_urls(target: String, doc_content: Option<String>) -> Vec<Url> {
        let image_urls = match doc_content {
            Some(content) => {
                let mut urls = Vec::new();
                let soup = Soup::new(&content);

                for img in soup.tag("img").find_all() {
                    let image_url = img.get("src").expect("Couldn't find `src` attribute");

                    // Make sure URL is absolute and add it to urls vector;
                    match Url::parse(&image_url) {
                        Ok(url) => {
                            urls.push(url);
                        },  // Already absolute, send to vector
                        Err(e) => {
                            match e {
                                ParseError::RelativeUrlWithoutBase => {
                                    // println!("Relative URL: {}", &image_url);
                                    let target_url = Url::parse(&target);  // Second parsing
                                    let absolute_url = target_url.unwrap().join(&image_url)
                                        .expect("Can't make absolute URL of image");

                                    // println!("absolute URL: {}", &absolute_url);
                                    urls.push(absolute_url);
                                },  // Relative URL error
                                _ => {
                                    // println!("errore: {}", e);
                                    return Vec::new()  // TODO: error
                                }  // Unknown error
                            };  // match error
                        }  // if error
                    }  // match url parse
                };  // for img in soup

                // println!("Image URLS: {:?}", urls);
                urls
            },
            None => {
                Vec::new()  // No images
            } // Empty vector
        };

        image_urls
    }  // extract_image_urls

}  // Document
