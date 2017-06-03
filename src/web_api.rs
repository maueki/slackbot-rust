
const API_URL: &'static str = "https://slack.com/api/";

use std::io::Read;

use hyper::Url;
use hyper::client::*;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use std::env;

pub fn get(method: &str) -> String {

    let tls = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(tls);
    let client = Client::with_connector(connector);

    let mut url = Url::parse(&(API_URL.to_string() + method)).unwrap();

    let token = env::var("SLACKBOT_TOKEN").unwrap();
    url.query_pairs_mut().append_pair("token", &token);

    let mut res = client.get(url).send().unwrap();

    let mut buf =  String::new();
    res.read_to_string(&mut buf).unwrap();

    println!("res = {:?}", buf);

    return "hoge".to_string();
}
