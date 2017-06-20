
const API_URL: &'static str = "https://slack.com/api/";

use std::io::Read;
use std::str;

use curl::easy::Easy;

use hyper::Url;

use std::env;
use serde_json;
use serde_json::{Value};

pub fn get(method: &str) -> String {

    let mut handle = Easy::new();
    // handle.proxy("http://yourproxyhost");

    let mut url = Url::parse(&(API_URL.to_string() + method)).unwrap();

    let token = env::var("SLACKBOT_TOKEN").unwrap();
    url.query_pairs_mut().append_pair("token", &token);

    handle.url(url.as_str()).unwrap();

    let mut buf = String::new();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|body| {
            buf.push_str(str::from_utf8(body).unwrap());
            Ok(body.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    let team_state: Value = serde_json::from_str(&buf).unwrap();

    let ws_url = &team_state["url"];

    return ws_url.to_string().replace("\"", "");
}
