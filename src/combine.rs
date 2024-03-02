pub mod parse {
    use std::fmt::{Error};
    use std::process::Command;
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::prelude::*;
    use crate::cmd::cmd::combine_ts;
    use crate::common::now;
    use crate::m3u8::HlsM3u8Method;
    use openssl::symm::{decrypt, Cipher};
    use reqwest::Client;

    pub fn get_reg_files(reg_name: String, reg_start: i32, reg_end: i32) -> Result<Vec<String>, Error> {
        let mut files = vec![];
        for i in reg_start..=reg_end {
            let file = reg_name.replace("(.*)", &i.to_string());
            files.push(file);
        }
        Ok(files)
    }

    pub fn get_reg_file_name(reg_name: String) -> String {
        return reg_name.replace("(.*)", "");
    }

    fn get_temp_file() -> String {
        if let Ok(dir) = tempdir() {
            if let Some(a) = dir.path().join(format!("{}.txt", now())).to_str() {
                return a.to_owned();
            }
        }
        return String::default();
    }

    pub fn to_files() -> Result<String, Error> {
        // let file = get_temp_file();
        let file = format!("{}.txt", now());
        Ok(file)
    }

    pub fn white_to_files(files: Vec<String>, file_name: String) -> Result<bool, Error> {
        println!("{}", file_name.clone());
        let mut file = File::create(file_name).expect("无法创建文件");
        for num in files {
            let str = format!("file \'{}\'", num);
            file.write_all(str.as_bytes()).expect("写入文件失败");
            file.write_all(b"\n").expect("写入文件失败");
        }
        Ok(true)
    }

    async fn combine_without_crypto(reg_name: String, reg_start: i32, reg_end: i32, target_name: String) -> Result<bool, Error> {
        let files = get_reg_files(reg_name.clone(), reg_start, reg_end).expect("解析失败");
        let file_name = to_files().expect("生成文件失败");
        let mut target = String::default();
        if target_name.is_empty() {
            target = format!("{}", get_reg_file_name(reg_name.to_owned()));
        } else {
            target = format!("{}", target_name.clone());
        }
        white_to_files(files.clone(), file_name.clone()).expect("写入文件失败");
        let res = combine_ts(file_name.clone(), target).expect("合并文件失败");
        Ok(res)
    }

    async fn combine_with_simple_aes(reg_name: String, reg_start: i32, reg_end: i32, target_name: String, key: String, iv: String, sequence: i32) -> Result<bool, Error> {
        // 未实现
        Ok(false)
    }

    async fn decrypt_video_file(key: &[u8], iv: &[u8], sequence_number: u8, segment_url: &str) {
        let mut file = File::open(segment_url).expect("文件不存在");
        let mut file_data = Vec::new();
        let _ = file.read_to_end(&mut file_data).expect("读文件失败");

        let cipher = Cipher::aes_128_cbc();
        let decrypted_data = decrypt(
            cipher,
            &key,
            Some(&iv),
            &file_data.as_slice(),
        ).expect("解析失败");

        let file_name = format!("decrypted-{}.ts", sequence_number);
        let mut file = match File::create(&file_name) {
            Err(why) => panic!("couldn't create: {}", why),
            Ok(file) => file,
        };

        match file.write_all(&decrypted_data) {
            Err(why) => panic!("couldn't write to: {}", why),
            Ok(_) => println!("successfully wrote to {}", &file_name),
        }
    }

    async fn combine_with_aes_128(reg_start: i32, reg_end: i32, target_name: String, key: String, iv: String, sequence: i32) -> Result<bool, Error> {
        println!("pass key {}, iv {}", key.clone(), iv.clone());
        let mut key_file = File::open(format!("{}.key", key.clone())).expect("key 文件不存在");
        let mut key_data = "".to_string();
        let _ = key_file.read_to_string(&mut key_data).expect("读key文件失败");
        println!("key is {}", key_data);
        let files = get_reg_files("(.*).ts".to_string(), reg_start, reg_end).expect("解析失败");
        let mut start_se = sequence;
        for i in files.clone() {
            let _ = decrypt_video_file(key_data.clone().as_bytes(), iv.clone().as_bytes(), start_se as u8, &i).await;
            start_se += 1;
        }
        return combine_without_crypto("decrypted-(.*).ts".to_string(), reg_start, reg_end, target_name).await;
    }

    pub async fn handle_combine_ts(reg_name: String, reg_start: i32, reg_end: i32, target_name: String,
                                   method: Option<HlsM3u8Method>, key: String, iv: String, sequence: i32,
    ) -> Result<bool, Error> {
        match method {
            Some(HlsM3u8Method::Aes128) => {
                println!("aes 128 decode");
                combine_with_aes_128(reg_start, reg_end, target_name, key.clone(), iv.clone(), sequence).await
            }
            Some(HlsM3u8Method::SampleAes) => {
                println!("simple aes");
                combine_with_simple_aes(reg_name, reg_start, reg_end, target_name, key.clone(), iv.clone(), sequence).await
            }
            None => {
                println!("no crypto");
                combine_without_crypto(reg_name, reg_start, reg_end, target_name).await
            }
        }
    }
}