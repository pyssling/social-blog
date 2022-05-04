use pulldown_cmark::{html, Options, Parser};
use chrono::{DateTime, Utc};
use json::object;

pub enum BlogContentsFormat {
    Markdown,
}

pub struct BlogEntry {
    title : String,
    contents : String,
    contents_format : BlogContentsFormat,
    timestamp : chrono::NaiveDateTime,
    html_contents : String
}

impl BlogEntry {
pub fn new(title : String, contents: String, contents_format : BlogContentsFormat, timestamp : chrono::NaiveDateTime ) -> Self
{
    let html_contents = generate_contents_html(&contents);
    return BlogEntry{
        title,
        contents,
        contents_format,
        timestamp,
        html_contents
    };
}
}

fn generate_contents_html(contents : &String) -> String
{
    let mut options : Options = Options::empty();
    options.set(Options::ENABLE_STRIKETHROUGH, true);
    let parser = Parser::new_ext(contents.as_str(), options);
    let mut html_output : String = String::with_capacity(contents.len() * 3/2);
    html::push_html(&mut html_output, parser);
    return html_output;
}

pub fn blog_entry_to_json( entry : crate::BlogEntry )
{

    let contents_format = match entry.contents_format {
        BlogContentsFormat::Markdown => "markdown",
    };

    let json_entry = object!{
        title : entry.title,
        contents : entry.contents,
        contents_format : contents_format,
        timestamp : entry.timestamp.timestamp(),
    };

    json_entry.dump();
}