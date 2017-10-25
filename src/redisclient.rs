extern crate redis;
use redis::Commands;
use std::env;

const KEY : &str = "twitter_bookmark";

fn get_connection() -> redis::Connection {
    // connect to redis
    let url : &str = &(env::var("REDIS_URL").unwrap());
    let client = redis::Client::open(url);
    client.unwrap().get_connection().unwrap()
}

pub fn get() -> Option<u64> {
    let x : redis::RedisResult<isize> = get_connection().get(KEY);
    x.ok().map(|x| x as u64)
}

pub fn set(value: u64) {
    let conn : redis::Connection = get_connection();
    let _ : () = conn.set(KEY, value).unwrap();
}
