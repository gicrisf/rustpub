extern crate soup;
extern crate url;
extern crate image;

use soup::prelude::*;
use url::{Url, ParseError};
use std::path::PathBuf;
// use image::io::Reader;

#[derive(Debug)]
pub struct ImgMeta {
    pub url: Option<Url>,
    pub filename: Option<String>,
    pub extension: Option<String>,
    pub local_path: Option<String>,
}

pub struct ImgProc {
    target: String,
    tmp_dir_path: PathBuf,
    bw: bool,
    max_size: u32,
}

impl ImgProc {
    pub fn new(target: String, tmp_dir_path: PathBuf, max_size: u32, bw: bool) -> Self {
        Self {
            target,
            tmp_dir_path,
            bw,
            max_size,
        }
    }

    fn absolute_checker(&self, img_url: &str) -> Url {
        // Make sure URL is absolute and add it to urls vector;
        match Url::parse(&img_url) {
            Ok(url) => { url },  // Already absolute, send to vector
            Err(e) => {
                match e {
                    ParseError::RelativeUrlWithoutBase => {
                        // println!("Relative URL: {}", &image_url);
                        let target_url = Url::parse(&self.target);  // Second parsing
                        let absolute_url = target_url.unwrap().join(&img_url)
                            .expect("Can't make absolute URL of image");

                        absolute_url
                    },  // Relative URL error
                    _ => panic!("{:?}", e)
                }  // match error
            }  // if error
        }  // match url parse
    }

    fn img_opt(&self, original: &[u8]) -> anyhow::Result<Vec<u8>> {
        let img = image::load_from_memory(original)
            .expect("Image optimization failed in loading original file");

        let mut thumb = img.thumbnail(self.max_size as u32, self.max_size as u32);

        if self.bw {
            thumb = image::DynamicImage::ImageLuma8(thumb.into_luma8());
        };

        let mut bytes: Vec<u8> = Vec::new();
        thumb.write_to(&mut bytes, image::ImageOutputFormat::Png)?;
        Ok(bytes)
    }

    pub async fn extract(&self, content: String) -> anyhow::Result<Vec<ImgMeta>> {
        let mut metas = Vec::new();
        let soup = Soup::new(&content);

        for img in soup.tag("img").find_all() {
            // Get image absolute URL
            let image_url = img.get("src").expect("Couldn't find `src` attribute");
            let image_url = self.absolute_checker(&image_url);

            // Make HTTP request for target file
            let response = reqwest::get(image_url.as_str()).await?;

            // Getting filename
            let filename = response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp_img.jpg")
                .to_string();

            // println!("Image to download: '{:?}'", filename);

            // Locate destination
            let local_abs_path = self.tmp_dir_path.join(&filename);

            let local_abs_pathstr = local_abs_path
                .clone()
                .into_os_string()
                .into_string()
                .expect("Error converting local abs path to string.");

            // Get extension
            let ext: Option<String> = match local_abs_path.extension() {
                Some(file_ext) => {
                    let extension = file_ext
                        .to_os_string()
                        .into_string()
                        .unwrap();

                    // let point_ext = format!(".{}", extension.clone());
                    // let new_filename = &filename.replace(&point_ext, "");
                    // filename = new_filename.to_string();

                    Some(extension)
                },
                None => None  // No extension
            };

            // Copy file to temp dir
            let mut destination = std::fs::File::create(local_abs_path.clone())?;
            let bytes = &response.bytes().await?[..];

            // (very UGLY) optimization
            let guess = image::io::Reader::open(local_abs_pathstr.clone())?
                .with_guessed_format()?
                .into_dimensions();

            let dimensions = match guess {
                Ok(guessed) => {
                    println!("{:?}", &guessed);
                    guessed
                },
                Err(_) => (0,0)
            };

            let thumbnail;

            if (dimensions.0 > self.max_size) || (dimensions.1 > self.max_size) {
                thumbnail = self.img_opt(bytes)?;
            } else {
                thumbnail = bytes.to_vec();
            }

            let mut bytes = &thumbnail[..];

            // Save on disk
            std::io::copy(&mut bytes, &mut destination).expect("Failed to copy image to dest.");

            // Return meta object
            let meta = ImgMeta {
                url: Some(image_url),
                filename: Some(filename),
                extension: ext, // Some(file_ext)
                local_path: Some(local_abs_pathstr),
            };

            metas.push(meta);
        };  // for img in soup

        // println!("Image metas: {:?}", metas);
        Ok(metas)
    }  // extract
}
