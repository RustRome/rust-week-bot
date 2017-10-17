extern crate egg_mode;

extern crate tokio_core;
extern crate slack_hook;
use slack_hook::{Slack, PayloadBuilder};

use tokio_core::reactor::Core;

use egg_mode::user::UserID;
use egg_mode::tweet::Timeline;

fn get_token() -> egg_mode::Token {

    let con_token = egg_mode::KeyPair::new("", "");
    let access_token = egg_mode::KeyPair::new("", "");
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };

    return token;

}

fn get_tweets (username: &str) -> Vec<String> {

    let user = UserID::from(username);

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let token = get_token();

    let mut tweetz : Timeline = egg_mode::tweet::user_timeline(user, false, false, &token, &handle);

    let mut vec = Vec::new();

    let tweets = &core.run(tweetz.start()).unwrap();

    for tweet in tweets {
        vec.push(tweet.text.clone());
    }

    return vec;
}

fn main() {

    for tweet in get_tweets("ThisWeekInRust") {
        slack_msg(tweet);
    }

}


fn slack_msg(msg: String) {
    let slack = Slack::new("https://hooks.slack.com/services///").unwrap();
    let p = PayloadBuilder::new()
      .text(msg)
      .channel("#random")
      .username("This week in rust BOT")
      .icon_url("https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png")
      .build()
      .unwrap();

    let res = slack.send(&p);
    match res {
        Ok(()) => println!("ok"),
        Err(x) => println!("ERR: {:?}",x)
    }
}