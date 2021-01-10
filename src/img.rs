extern crate soup;
extern crate url;
// extern crate image;

use soup::prelude::*;
use url::{Url, ParseError};
// use image::io::Reader as ImageReader;

pub struct ImgMeta {
    pub url: Option<Url>,
    pub filename: Option<String>,
    pub extension: Option<String>,
    pub local_path: Option<String>,
}

pub fn extract_image_urls(target: String, doc_content: Option<String>) -> Vec<Url> {
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

pub async fn download_images(image_urls: Vec<Url>, target: String, tmp_dir_path: std::path::PathBuf) -> anyhow::Result<Vec<ImgMeta>> {
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
        let mut destination = std::fs::File::create(local_abs_path.clone())?;

        let mut bytes = &response.bytes().await?[..];

        std::io::copy(&mut bytes, &mut destination).expect("Failed to copy image to dest.");

        let meta = ImgMeta {
            url: Some(url),
            filename: Some(filename),
            extension: Some(ext),
            local_path: Some(local_abs_pathstr),
        };

        image_metas.push(meta);
    }

    Ok(image_metas)
}
