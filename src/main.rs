use isahc::prelude::*;
use serde_json::Value;
use std::io::{Error, ErrorKind};
use std::process::exit;

fn request_topics(url: &str) -> std::io::Result<Value> {
    let mut resp = isahc::get(url).unwrap();
    let status = resp.status();
    if status != 200 {
        return Err(Error::new(
                ErrorKind::Other,
                format!("request content error: {:?}", status),
        ));
    }

    let headers = resp.headers();
    match headers.get("X-Rate-Limit-Remaining") {
        Some(remaining) => {
            let remaining = remaining.to_str().unwrap().parse::<u8>().unwrap();
            if remaining == 0 {
                return Err(Error::new(
                        ErrorKind::Other,
                        "api limit remaining is exhausted",
                ));
            }
        }
        None => {
            return Err(Error::new(
                    ErrorKind::Other,
                    "parse header X-Rate-Limit-Remaining error",
            ));
        }
    }

    let text = resp.text().unwrap();
    let topics: Value = serde_json::from_str(&text).unwrap();
    Ok(topics)
}

fn main() {
    // get hot topics
    let hot_topics = match request_topics("https://www.v2ex.com/api/topics/hot.json") {
        Ok(topics) => topics,
        Err(err) => {
            println!("{:?}", err);
            exit(1);
        }
    };
    println!(">>> 最热主题");
    for i in 0.. {
        match hot_topics.get(i) {
            Some(value) => {
                println!("{}", value["title"].as_str().unwrap());
            }
            None => break,
        }
    }

    // get latest topics
    let latest_topics = match request_topics("https://www.v2ex.com/api/topics/latest.json") {
        Ok(topics) => topics,
        Err(err) => {
            println!("{:?}", err);
            exit(1);
        }
    };

    println!("\n>>> 最新主题");
    for i in 0.. {
        match latest_topics.get(i) {
            Some(value) => {
                println!("{}", value["title"].as_str().unwrap());
            }
            None => break,
        }
    }
}
