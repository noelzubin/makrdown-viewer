#[macro_use]
use web_view::*;
use std::env;
use pulldown_cmark::{Parser, Options, html};
use std::fs;

/// Return the HTML style tag
pub fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

/// Return the HTML script tag
pub fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}

fn main() -> Result<(), Box<Error>> {

    let mut args = env::args();
    args.next().unwrap();
    let filename: String = args.next().expect("no file provided");

    let contents = fs::read_to_string(&filename).unwrap();
    let contents: String = contents.parse().unwrap();


    let mut options = Options::empty();
    // options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&contents, options);

     let mut html_output = String::new();
     html::push_html(&mut html_output, parser);

     let html: String = vec![
         html_output.to_string(), 
         inline_style(include_str!("index.css")),
         inline_style(include_str!("hljs-default.css")),
         inline_script(include_str!("hljs.js")),
         "<script>hljs.initHighlightingOnLoad();</script>".to_string(),
    ].join("\n");


    let content = Content::Html(html.to_string());

    // let content Content::Url("https://en.m.wikipedia.org/wiki/Main_Page")

    web_view::builder()
        .title(&filename)
        .content(content)
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();

    Ok(())
}

