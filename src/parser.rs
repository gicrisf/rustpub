// extern crate lazy_static;

extern crate kuchiki;
use kuchiki::traits::*;

use crate::epub::Document;

pub struct KuchikiParser {}

impl KuchikiParser {
    pub fn parse(html: String) {
        let sink = kuchiki::parse_html().one(html);

        let mut title: String;

        for matched in sink.select("h1").unwrap() {
            let as_node = matched.as_node();
            let text_node = as_node.first_child().unwrap();
            let text = text_node.as_text().unwrap().borrow();
            title = text.to_string();
            println!("{}", title);
        };

        // Document {
        //     title,
        //     byline
        // }
    }
}
