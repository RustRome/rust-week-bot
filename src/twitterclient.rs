extern crate egg_mode;

use std::env;
use tokio_core::reactor::Core;
use egg_mode::user::UserID;
use egg_mode::tweet::Tweet;
use redisclient;

fn get_token() -> egg_mode::Token {

    let api_key = env::var("TWITTER_API_KEY").unwrap();
    let api_secret = env::var("TWITTER_API_SECRET").unwrap();

    let access_token = env::var("TWITTER_ACCESS_TOKEN").unwrap();
    let access_token_secret = env::var("TWITTER_ACCESS_TOKEN_SECRET").unwrap();

    let con_token = egg_mode::KeyPair::new(api_key, api_secret);
    let access_token = egg_mode::KeyPair::new(access_token, access_token_secret);
    egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    }

}

pub fn get_tweets (username: &str) -> Vec<Tweet> {

    let user = UserID::from(username);

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let token = get_token();
    let bookmark = redisclient::get();

    println!("bookmark: {:?}", bookmark);

    let twitter_future =
        egg_mode::tweet::user_timeline(user, false, false, &token, &handle)
            .call(bookmark, None);

    let tweets : Vec<Tweet> = core
        .run(twitter_future)
        .unwrap()
        .to_vec();

    match tweets.first().map(|tweet| tweet.id - 1) {
        Some(tweet_id) => {
            redisclient::set(tweet_id)
        }
        None => {
            println!("nessun bookmark");
        }
    }

    tweets

}