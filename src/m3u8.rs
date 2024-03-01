pub struct HlsM3u8 {
    pub key: String,
    pub list: Vec<String>,
    pub method: String,
    original_url: String,// SAMPLE-AES || AES-128
}

impl HlsM3u8 {
    pub fn new() -> HlsM3u8 {
        HlsM3u8 {
            key: "".to_string(),
            list: vec![],
            method: "".to_string(),
            original_url: "".to_string(),
        }
    }

    pub fn set_list(&mut self, list: Vec<String>) {
        self.list = list
    }

    pub fn set_original_url(&mut self, original_url: String) {
        self.original_url = original_url
    }

    pub fn set_method_and_key(&mut self, method: String, key: String) {
        self.method = method;
        self.key = key;
        self.convert_local_key();
    }

    fn convert_local_key(&mut self) {
        let key = "".to_string();

        // self.key = key;
    }
}

pub mod m3u8 {
    use std::fs::File;
    use std::io::Read;
    use crate::common::{download_file, is_url, now, replace_last_segment};
    use crate::m3u8::HlsM3u8;
    use regex::Regex;

    pub fn parse_local(local_file: String, target_url: String) -> HlsM3u8 {
        let mut data = File::open(local_file).expect("file not exists");
        let mut str = String::default();
        let _ = data.read_to_string(&mut str);
        str_to_urls(str, target_url.clone())
    }

    fn str_to_urls(str: String, url: String) -> HlsM3u8 {
        let mut hls_m3u8 = HlsM3u8::new();
        hls_m3u8.set_original_url(url.clone());
        let mut list = vec![];
        let arr = str.split("\n").into_iter();
        for i in arr {
            if !i.is_empty() {
                if !i.starts_with("#EXT") {
                    if is_url(i.to_string()) {
                        list.push(i.to_string());
                    } else {
                        if !url.is_empty() {
                            let new_url = replace_last_segment(&url, i);
                            list.push(new_url)
                        }
                    }
                } else {
                    if i.starts_with("#EXT-X-KEY") {
                        let method = get_method_from_regex(i);
                        let uri = get_uri_from_regex(i);
                        hls_m3u8.set_method_and_key(method, uri);
                    }
                }
            }
        }
        hls_m3u8.set_list(list);
        hls_m3u8
    }

    pub fn get_method_from_regex(str: &str) -> String {
        let regex = Regex::new(r"(?m)METHOD=(.*),").unwrap();

        // result will be an iterator over tuples containing the start and end indices for each match in the string
        let result = regex.captures_iter(str);

        for mat in result {
            return mat.get(1).expect("error").as_str().to_string();
        }
        "".to_string()
    }

    pub fn get_uri_from_regex(str: &str) -> String {
        let regex = Regex::new(r#"(?m)URI="(.*)""#).unwrap();

        // result will be an iterator over tuples containing the start and end indices for each match in the string
        let result = regex.captures_iter(str);

        for mat in result {
            return mat.get(1).expect("error").as_str().to_string();
        }
        "".to_string()
    }

    pub async fn parse_url(url: String) -> HlsM3u8 {
        let hls_m3u8 = HlsM3u8::new();
        let local_file = format!("./{}.m3u8", now());
        return match download_file(url.clone(), local_file.clone()).await {
            Ok(data) => {
                if data {
                    parse_local(local_file.clone().to_string(), url.clone())
                } else {
                    hls_m3u8
                }
            }
            _ => {
                hls_m3u8
            }
        };
    }
}