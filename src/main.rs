extern crate egg_mode;
extern crate tokio_core;
extern crate slack_hook;
extern crate dotenv;
extern crate redis;

use dotenv::dotenv;

mod publisher;
mod twitterclient;
mod redisclient;

use publisher::Publisher;

fn main() {

    dotenv().ok();

    let publisher = Box::new(publisher::ConsolePublisher);

    for tweet in twitterclient::get_tweets("ThisWeekInRust") {
        publisher.publish(tweet.text.clone());
    }

}
