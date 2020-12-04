extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_github_url(query: &str) -> String {
    if query == "gh" {
        let github_dotcom = "https://github.com";
        github_dotcom.to_string()
    } else if &query[..4] == "gh @" {
        let encoded_query = utf8_percent_encode(&query[4..], FRAGMENT).to_string();
        let github_url = format!("https://github.com/{}", encoded_query);
        github_url
    } else {
        construct_github_search_url(&query)
    }
}
pub fn construct_github_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(&query[3..], FRAGMENT).to_string();
    let github_url = format!("https://github.com/search?q={}", encoded_query);
    github_url
}
