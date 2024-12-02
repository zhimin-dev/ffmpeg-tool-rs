use std::fmt::Error;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

pub fn now() -> u64 {
    let now = SystemTime::now();
    return now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
}

pub fn is_url(_str: String) -> bool {
    let _url = &_str;
    let check_url = Url::parse(_url);
    return match check_url {
        Ok(_) => true,
        Err(_) => false,
    };
}

pub fn get_url_host(str: &str) -> Result<String, Error> {
    if str.is_empty() {
        return Ok("".to_string());
    }
    let binding = Url::parse(str).unwrap();
    let url = binding.host_str().expect("解析url失败");
    Ok(url.to_string())
}

pub fn replace_last_segment(url: &str, replacement: &str) -> String {
    let mut components: Vec<&str> = url.split('/').collect();
    if let Some(last_segment) = components.last_mut() {
        *last_segment = replacement;
    }
    components.join("/")
}

pub async fn download_file(url: String, file_name: String) -> Result<bool, Error> {
    let resp = reqwest::get(&url).await.expect("get url data error");
    let bytes = resp.bytes().await.expect("get data error");
    fs::write(file_name.clone(), &bytes).expect("write file error");
    Ok(true)
}
