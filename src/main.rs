extern crate egg_mode;
extern crate tokio_core;
extern crate slack_hook;
extern crate dotenv;
extern crate redis;

use dotenv::dotenv;

mod slackclient;
mod twitterclient;
mod redisclient;

fn main() {

    dotenv().ok();

    for tweet in twitterclient::get_tweets("ThisWeekInRust") {
        slackclient::slack_msg(tweet.text.clone());
    }

}
