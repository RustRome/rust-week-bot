use slack_hook::{Slack, PayloadBuilder};
use std::env;

pub trait Publisher {
    fn publish(&self, message: String);
}

pub struct SlackPublisher;

impl Publisher for SlackPublisher {
    fn publish(&self, message: String) {

      let url = env::var("SLACK_WEBHOOKS")
        .expect("no slack webhook URL defined");

      let url_str : &str = &(url);
      let slack = Slack::new(url_str).expect("no slack client");

      let payload = PayloadBuilder::new()
        .text(message)
        .channel("#random")
        .username("This week in rust BOT")
        .icon_url("https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png")
        .build()
        .expect("cannot create payload");

      let res = slack.send(&payload);
      match res {
          Ok(()) => println!("ok"),
          Err(x) => println!("ERR: {:?}",x)
      }
    }
}

pub struct ConsolePublisher;

impl Publisher for ConsolePublisher {
    fn publish(&self, message: String) {
        println!("message: {}", message);
    }
}
