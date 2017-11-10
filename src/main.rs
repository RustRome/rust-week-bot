extern crate egg_mode;
extern crate tokio_core;
extern crate slack_hook;
extern crate dotenv;
extern crate redis;
extern crate reqwest;
extern crate regex;
extern crate futures;
extern crate hyper;
extern crate select;

use dotenv::dotenv;
use publisher::Publisher;

mod publisher;
mod twitterclient;
mod redisclient;
mod scraper;

fn main() {

  dotenv().ok();

  let publisher = publisher::SlackPublisher;

  for tweet in twitterclient::get_tweets("ThisWeekInRust") {

    let blogz =  scraper::scrape_tweet(tweet.text);
    publisher.publish(blogz);
  }


}
