extern crate redis;
use redis::Commands;
use std::env;

const KEY : &str = "twitter_bookmark";

fn get_connection() -> redis::Connection {
  // connect to redis
  let url : &str = &(env::var("REDIS_URL").expect("must define a Redis URL"));
  let client = redis::Client::open(url);
  let error_message : &str = &(format!("cannot connect to redis {}", url));
  client.expect("cannot create client").get_connection().expect(error_message)
}

pub fn get() -> Option<u64> {
    let x : redis::RedisResult<isize> = get_connection().get(KEY);
    x.ok().map(|x| x as u64)
}

pub fn set(value: u64) {
    let conn : redis::Connection = get_connection();
    let _ : () = conn.set(KEY, value).expect(&(format!("cannot set value {} {}", KEY, value)));
}
