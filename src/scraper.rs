extern crate reqwest;
extern crate regex;
extern crate select;

use std::io::Read;
use regex::Regex;

use select::document::Document;
use select::node::Node;
use select::predicate::Name;

fn get_html(u: &str) -> String {
  let mut resp = reqwest::get(u).unwrap();
  assert!(resp.status().is_success());

  let mut content: String = String::new();
  let result: Result<usize, _> = resp.read_to_string(&mut content);

  println!("{} bytes", result.unwrap());

  content
}

struct Item {
  text: String,
  link: String
}


// format!("[{}]({})", node.text(), node.attr("href").expect("no href attribute")
fn extract_content(html: String) -> String {
  Document::from(html.as_str())
    .find(Name("ul"))
    .next()
    .unwrap()
    .find(Name("a"))
    .map(|node: Node| Item{text: node.text(), link: node.attr("href").expect("no href").to_string()})
    .fold(String::new(), |acc, item| acc + &format!("[{}]({})\n", item.text, item.link))
}


pub fn scrape_tweet(text: String) -> String {
  let re: Regex = Regex::new(r"(http.*) .*").unwrap();

  let my_url = re.captures(&(text))
    .expect("url not found")
    .get(1)
    .expect("expected an URL")
    .as_str();

  let html: String = get_html(my_url);
  extract_content(html)
}
