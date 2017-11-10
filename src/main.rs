extern crate egg_mode;
extern crate tokio_core;
extern crate slack_hook;
extern crate dotenv;
extern crate redis;
extern crate regex;
extern crate reqwest;

extern crate futures;
extern crate hyper;

extern crate select;
use select::document::Document;
use select::node::Node;
use select::predicate::{Predicate, Attr, Class, Name};

use std::io::{Read};


use regex::Regex;

use dotenv::dotenv;

mod publisher;
mod twitterclient;
mod redisclient;

use publisher::Publisher;

use reqwest::{Response};


fn main() {

  dotenv().ok();

  let publisher = publisher::SlackPublisher;

  let re = Regex::new(r"(http.*) .*").unwrap();

  for tweet in twitterclient::get_tweets("ThisWeekInRust") {

    let my_url = re.captures(&(tweet.text))
      .expect("url not found")
      .get(1)
      .expect("expected an URL")
      .as_str();


    let html: String = get_html(my_url);
    let blogz : String = extract_content(html);
    publisher.publish(format!("<ul>{}</ul>", blogz));
  }


    fn get_html (u: &str) -> String {

        let mut resp = reqwest::get(u).unwrap();
        assert!(resp.status().is_success());

        let mut content : String = String::new();
        resp.read_to_string(&mut content);

        // println!("{}", content);

        content
    }

    fn extract_content(html: String) -> String {
        let document = Document::from(html.as_str());
        let node: Node = document.find(Name("ul")).next().unwrap();
        node.inner_html()
    }

}
