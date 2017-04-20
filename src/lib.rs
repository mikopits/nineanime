extern crate hyper;
extern crate hyper_native_tls;
extern crate kuchiki;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use serde_json::Value;
use std::sync::Arc;
use std::io::Read;

use hyper::{Client, Url};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use kuchiki::traits::*;

pub use self::error::{Error, Result};
pub use self::query::*;

mod error;
mod query;

//static FILTER_URL: &'static str = "https://9anime.to/filter";
static SEARCH_URL: &'static str = "https://9anime.to/search";

// info json response:
// {\"grabber\":\"https:\\/\\/9anime.to\\/grabber-api\\/\",\"params\":{\"id\":
// \"oxnxx5\",\"token\":\"IFmEgNFcXOoGvJbTmZ6BOB6E0M54yvgN5keGw7RT0xtD7cM9ogDFc
// cwalrzH6o6wuHqGU2G60eHu6gNAvgoBDhnrIEqwbrMFCVBXtMZnRpaGHxPNnetmTMRKk90=\",
// \"options\":\"LhyCgccIA7AF@9rJhcmTKFDS2t4zyugor1OVyKgG\\/U4QltZJgwjbWMpb4qHf
// 1bvXxwaN\"},\"target\":\"\",\"type\":\"direct\",\"name\":\"06\",
// \"subtitle\":\"\"}
static INFO_URL: &'static str = "https://9anime.to/ajax/episode/info";
static GRABBER_API: &'static str = "http://9anime.to/grabber-api/";

// <div class="item"> == $0
//     <a href="https://9anime.to/watch/tokyo-mew-mew-dub.rg5n"
//     class="poster" data-tip="ajax/film/tooltip/rq5n?v21490167890"
//     title>
//         <img src="https://images2-focus-opensocial.googleusercontent
//         .com/gadgets/proxy?container=focus&gadget=a&no_expand=1&refr
//         esh=604800&url=http://2.bp.blogspot.com/-fk-4K78TomQ/WNIoUK-
//         OCXI/AAAAAAABgMQ/2lu5oacBGE0/s0/" alt="Tokyo Mew Mew (Dub)">
//         == $0
//      </a>
//      <a href="https://9anime.to/watch/tokyo-mew-mew-dub.rq5n"
//      class="name">Tokyo Mew Mew (Dub)</a> == $0
//      <div class="status"> 26/26 </div>
//      <div class="lang">DUB</div>
// </div>
//static COL_CLASS: &'static str = ".col-lg-4";
static POSTER_SELECTOR: &'static str = ".poster";
static THUMB_SELECTOR: &'static str = r#"img[src$="/s0/"]"#;

// <div class=\ "server row\" data-type=\ "direct\">
// <label class=\ "name col-md-3 col-sm-4\"> <i class=\ "fa fa-server\"></i> Server F2 </label>
//     <div class=\ "col-md-21 col-sm-20\">
//     <ul class=\ "episodes range active\" data-range-id=\ "0\">
//     <li> <a class=\ "active\" data-id=\ "k3j3x4\" data-base=\ "1\" data-comment=\ "1\" data-toggle=\ "tooltip\" data-title=\ "Mar 22, 2017 - 07:37\" href=\ "/watch/tokyo-mew-mew-dub.rq5n/k3j3x4\">01</a> </li>
//     <li> <a data-id=\ "2p4p02\" data-base=\ "2\" data-comment=\ "2\" data-toggle=\ "tooltip\" data-title=\ "Mar 22, 2017 - 07:37\" href=\ "/watch/tokyo-mew-mew-dub.rq5n/2p4p02\">02</a> </li>
//     <li> <a data-id=\ "zx6xxm\" data-base=\ "3\" data-comment=\ "3\" data-toggle=\ "tooltip\" data-title=\ "Mar 22, 2017 - 07:37\" href=\ "/watch/tokyo-mew-mew-dub.rq5n/zx6xxm\">03</a> </li>
//     ...
//     <li> <a data-id=\ "7292mj\" data-base=\ "26\" data-comment=\ "26\" data-toggle=\ "tooltip\" data-title=\ "Mar 22, 2017 - 07:37\" href=\ "/watch/tokyo-mew-mew-dub.rq5n/7292mj\">26</a> </li>
//     </ul>
//     </div>
// </div>
static ID_SELECTOR: &'static str = r#"a[href^="/watch/"]"#;

macro_rules! try_parse {
    ($e:expr) => {
        match $e {
            Some(e) => e,
            None => {
                error!("Failed to parse");
                return Err(::error::Error::ParseError)
            }
        }
    }
}

#[derive(Debug)]
pub struct Anime {
    client: Arc<Client>,
    pub url: String,
    pub thumb: String,
    pub name: String
}

#[derive(Debug, Deserialize)]
pub struct Files {
    pub data: Vec<FileData>
}

impl Files {
    pub fn default(&self) -> FileData {
        //FIXME: Just threw up in my mouth a little
        self.data.iter()
            .filter(|&f| f.default)
            .take(1)
            .cloned()
            .collect::<Vec<FileData>>()
            .first()
            .unwrap()
            .clone()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct FileData {
    #[serde(default="Default::default")]
    pub default: bool,
    pub file: String,
    pub label: String,
    #[serde(rename="type")]
    pub ext: String
}

impl Anime {
    pub fn episode_id(&self, episode: usize) -> Result<String> {
        let url = try!(Url::parse(&self.url));
        let res = try!(self.client.get(url).headers(user_agent()).send());
        let doc = try!(kuchiki::parse_html().from_http(res));

        let mut ep_matches = match doc.select(ID_SELECTOR) {
            Ok(m) => m,
            Err(_) => return Err(::error::Error::ParseError)
        };

        let ep_server_f2 = try_parse!(ep_matches.nth(episode-1));
        let ep_node = ep_server_f2.as_node();
        let ep_ele = try_parse!(ep_node.as_element());
        let ep_attr = ep_ele.attributes.borrow();
        let ep_id = try_parse!(ep_attr.get("data-id"));
        debug!("ep_id: {:?}", ep_id);
        let ep_no = try_parse!(ep_attr.get("data-base"));
        if ep_no != &episode.to_string() {
            error!("Could not find episode {}", episode);
            return Err(::error::Error::ParseError)
        }
        Ok(ep_id.to_string())
    }

    pub fn files(&self, episode: usize) -> ::Result<Files> {
        let id = try!(self.episode_id(episode));
        let raw_url = format!("{}?id={}", INFO_URL, id);
        let parsed_url = try!(Url::parse(&raw_url));
        let mut res = try!(self.client.get(parsed_url)
                       .headers(user_agent()).send());
        let mut buf = String::new();
        try!(res.read_to_string(&mut buf));
        let v_t: Value = try!(serde_json::from_str(&buf));
        let token = &v_t["params"]["token"];
        debug!("Got token {} for id {}", token, id);
        let grab_url = format!("{}?id={}&token={}", GRABBER_API, id, token);
        let parsed_grab_url = try!(Url::parse(&grab_url));
        res = try!(self.client.get(parsed_grab_url)
                   .headers(user_agent()).send());
        buf = String::new();
        try!(res.read_to_string(&mut buf));
        let f: Files = try!(serde_json::from_str(&buf));

        Ok(f)
    }
}

pub fn search(keyword: &str) -> ::Result<Vec<Anime>> {
    let raw_url = format!("{}?keyword={}", SEARCH_URL,
                          keyword.to_string().replace(" ", "+"));
    let parsed_url = try!(Url::parse(&raw_url));

    let ssl = try!(NativeTlsClient::new());
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);
    let res = try!(client.get(parsed_url).headers(user_agent()).send());
    let doc = try!(kuchiki::parse_html().from_http(res));

    let mut urls: Vec<String> = Vec::new();
    let poster_matches = match doc.select(POSTER_SELECTOR) {
        Ok(m) => m,
        Err(_) => return Ok(Vec::new())
    };

    for poster_match in poster_matches {
        let poster_node = poster_match.as_node();
        let poster_ele = try_parse!(poster_node.as_element());
        let poster_attr = poster_ele.attributes.borrow();
        let url = try_parse!(poster_attr.get("href"));
        urls.push(url.to_string());
    }

    let mut thumbs: Vec<String> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    let thumb_matches = match doc.select(THUMB_SELECTOR) {
        Ok(m) => m,
        Err(_) => return Ok(Vec::new())
    };

    for thumb_match in thumb_matches {
        let thumb_node = thumb_match.as_node();
        let thumb_ele = try_parse!(thumb_node.as_element());
        let thumb_attr = thumb_ele.attributes.borrow();
        let thumb = try_parse!(thumb_attr.get("src"));
        let name = try_parse!(thumb_attr.get("alt"));
        thumbs.push(thumb.to_string());
        names.push(name.to_string());
    }

    if !(urls.len() == names.len() && names.len() == thumbs.len()) {
        error!("url, name, and thumb length do not match");
        return Err(::error::Error::ParseError)
    }

    let ref_client = Arc::new(client);

    Ok((0..urls.len()).map(|i| Anime {
        client: ref_client.clone(),
        url: urls[i].to_string(),
        thumb: thumbs[i].to_string(),
        name: names[i].to_string()
    }).collect::<Vec<Anime>>())
}

fn user_agent() -> hyper::header::Headers {
    let mut headers = hyper::header::Headers::new();
    headers.set(hyper::header::UserAgent("nineanime-rs".to_string()));
    headers
}

#[cfg(test)]
mod tests {
    extern crate env_logger;

    #[test]
    fn search_test() {
        env_logger::init().unwrap();
        let animes = match ::search("Mew Mew") {
            Ok(v) => v,
            Err(e) => {
                error!("Got error in search: {:?}", e);
                assert!(false);
                return
            }
        };

        debug!("Got animes: {:?}", animes);
        assert!(animes.len() > 0);
        let mew_mew_dub = animes.first().unwrap();
        assert_eq!(mew_mew_dub.url, "https://9anime.to/watch/tokyo-mew-mew-dub.rq5n".to_string());
        assert_eq!(mew_mew_dub.thumb, "https://images2-focus-opensocial.googleusercontent.com/gadgets/proxy?container=focus&gadget=a&no_expand=1&refresh=604800&url=http://2.bp.blogspot.com/-fk-4K78TomQ/WNIoUK-OCXI/AAAAAAABgMQ/2lu5oacBGE0/s0/".to_string());
        assert_eq!(mew_mew_dub.name, "Tokyo Mew Mew (Dub)".to_string());
        let files = mew_mew_dub.files(6).expect("Failed to get download link");
        assert!(files.default().ext == "mp4".to_string());
    }
}
