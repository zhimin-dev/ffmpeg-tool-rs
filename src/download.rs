use crate::common::{download_file, now};
use tokio::runtime::Runtime;
use std::fs;

struct VideoTs {
    index: i32,
    url: String,
}

impl VideoTs {
    pub fn new() -> VideoTs {
        VideoTs { index: 0, url: "".to_string() }
    }

    pub fn set(&mut self, index: i32, url: String) {
        self.index = index;
        self.url = url.clone();
    }
}

pub mod download {
    use std::fmt::{Error};
    use std::fs::File;
    use std::process::Command;
    use std::sync::{Arc, mpsc, Mutex};
    use std::thread;
    use crate::combine::parse::{get_reg_files, handle_combine_ts};
    use crate::common::{is_url, now};
    use crate::download::{download_ts_file, VideoTs};
    use crate::m3u8::m3u8::{parse_local, parse_url};
    use std::fs;
    use std::env;
    use std::path::Path;
    use crate::m3u8::HlsM3u8Method;

    pub async fn fast_download(url: String, _file_name: String, folder: String, concurrent: i32) -> Result<bool, Error> {
        let mut hls_m3u;
        if is_url(url.clone()) {
            hls_m3u = parse_url(url.clone(), folder.clone()).await;
        } else {
            hls_m3u = parse_local(url.clone(), String::default(), folder.clone()).await;
        }
        let mut ts_list = vec![];
        let mut ts_index = 0;
        for x in &hls_m3u.list {
            let mut ts = VideoTs::new();
            ts.set(ts_index, x.clone());
            ts_list.push(ts);
            ts_index += 1;
        }
        let total = ts_list.len();
        // 分批下载文件
        let (tx, rx) = mpsc::channel();
        let (data_tx, data_rx) = mpsc::channel();
        let new_data_rx = Arc::new(Mutex::new(data_rx));

        for _i in 0..concurrent {
            let tx_clone = tx.clone();
            let data_rx_clone = Arc::clone(&new_data_rx);

            thread::spawn(move || loop {
                let item = {
                    let rx_lock = data_rx_clone.lock().unwrap();
                    match rx_lock.recv() {
                        Ok(item) => item,
                        Err(_) => break,
                    }
                };
                let result = download_ts_file(item);
                tx_clone.send(result).unwrap();
            });
        }
        for value in ts_list {
            data_tx.send(value).unwrap();
        }
        drop(tx); // 发送完成后关闭队列
        let mut i = 0;
        loop {
            if i == total {
                break;
            }
            let result = rx.recv();
            match result {
                Ok(data) => {
                    i += 1;
                }
                Err(_e) => {}
            }
        }
        println!("----download files finished");
        return handle_combine_ts(String::from("(.*).ts"), 0,
                                 (total - 1) as i32, _file_name.clone(), hls_m3u.method, folder.clone(), hls_m3u.iv, hls_m3u.sequence).await;
    }

    pub fn create_folder(folder: String) -> Result<bool, Error> {
        // 检查文件夹是否存在
        if !fs::metadata(folder.clone()).is_ok() {
            // 文件夹不存在，创建文件夹
            match fs::create_dir(folder.clone()) {
                Ok(_) => {
                    Ok(true)
                }
                Err(e) => {
                    println!("创建文件夹时出错：{}", e);
                    Ok(false)
                }
            }
        } else {
            Ok(true)
        }
    }

    pub fn get_file_name(file_name: String) -> String {
        let mut target = file_name;
        if target.is_empty() {
            target = format!("{}.mp4", now());
        }
        target
    }
}

fn download_ts_file(video_ts: VideoTs) -> bool {
    let download_file_name = format!("./{}.ts", video_ts.index);
    match fs::metadata(download_file_name.clone()) {
        Ok(_) => {
            println!("file {} exists", video_ts.url.clone());
            true
        }
        Err(_) => {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                println!("download ts file {}", video_ts.url.clone());
                let res = download_file(video_ts.url.clone(), download_file_name).await;
                return match res {
                    Ok(data) => {
                        data
                    }
                    _ => {
                        false
                    }
                };
            })
        }
    }
}