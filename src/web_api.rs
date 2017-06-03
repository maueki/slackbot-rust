
const API_URL: &'static str = "https://slack.com/api/";

use hyper::client::*;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

pub fn get(method: &str) -> String {

    let tls = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(tls);
    let client = Client::with_connector(connector);

    let url = API_URL.to_string() + method;
    let res = client.get(&url).send().unwrap();

    println!("res = {:?}", res);

    return url;
}
