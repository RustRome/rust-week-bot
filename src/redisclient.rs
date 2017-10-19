extern crate redis;
use redis::Commands;

fn get_connection() -> redis::Connection {
    // connect to redis
    let client = redis::Client::open("redis://my-redis/");
    client.unwrap().get_connection().unwrap()
}

pub fn get() -> Option<u64> {
    let x : redis::RedisResult<isize> = get_connection().get("twitter_bookmark");
    x.ok().map(|x| x as u64)
}

pub fn set(value: u64) {
    let conn : redis::Connection = get_connection();
    let _ : () = conn.set("twitter_bookmark", value).unwrap();
}