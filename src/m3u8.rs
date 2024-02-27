pub mod m3u8 {
    use std::fs::File;
    use std::io::Read;
    use crate::common::{download_file, is_url, now, replace_last_segment};

    pub fn parse_local(local_file: String, target_url:String) -> Vec<String> {
        let mut data = File::open(local_file).expect("file not exists");
        let mut str = String::default();
        let _ = data.read_to_string(&mut str);
        str_to_urls(str,target_url.clone())
    }

    fn str_to_urls(str: String, url: String) -> Vec<String> {
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
                }
            }
        }
        list
    }

    pub async fn parse_url(url: String) -> Vec<String> {
        let list = vec![];
        let local_file = format!("./{}.m3u8", now());
        return match download_file(url.clone(), local_file.clone()).await {
            Ok(data) => {
                if data {
                    parse_local(local_file.clone().to_string(), url.clone())
                } else {
                    list
                }
            }
            _ => {
                list
            }
        }
    }
}