use slack_hook::{Slack, PayloadBuilder, SlackLink, AttachmentBuilder};
use slack_hook::SlackTextContent::{self, Link, Text};
use std::env;
use scraper::Page;

pub trait Publisher {
    fn publish(&self, page: Page);
}

pub struct SlackPublisher;

impl Publisher for SlackPublisher {
    fn publish(&self, page: Page) {

      let url = env::var("SLACK_WEBHOOKS")
        .expect("no slack webhook URL defined");

      let url_str : &str = &(url);
      let slack = Slack::new(url_str).expect("no slack client");

      let slack_messages : Vec<SlackTextContent> = page
        .links
        .into_iter()
        .map(|item| Link(SlackLink::new(&item.link, &item.text)))
        .flat_map(|item| vec![Text("- ".into()), item, Text("\n".into())]  )
        .collect();

      let payload = PayloadBuilder::new()
        .text(slack_messages.as_slice())
        .attachments(vec![AttachmentBuilder::new(page.title).color("#00FFCC").build().unwrap()])
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
    fn publish(&self, page: Page) {
        println!("message: {:?}", page);
    }
}
