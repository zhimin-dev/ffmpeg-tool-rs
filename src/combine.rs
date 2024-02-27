pub mod parse {
    use std::fmt::{Error};
    use std::process::Command;
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::prelude::*;
    use crate::common::now;

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
        let file = format!("./{}.txt", now());
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

    //ffmpeg -f concat -i input.txt -c copy output.mp4
    pub fn combine(file: String, target: String) -> Result<bool, Error> {
        let mut binding = Command::new("ffmpeg");
        let res = binding.arg("-f")
            .arg("concat")
            .arg("-i")
            .arg(file)
            .arg("-c")
            .arg("copy")
            .arg(target).output().unwrap().status;
        if res.success() {
            Ok(true)
        } else {
            println!("{}", res.to_string());
            Ok(false)
        }
    }

    //ffmpeg -f concat -safe 0 -i filelist.txt -c copy output.mp4
    pub fn combine_ts(file:String, target:String) -> Result<bool, Error> {
        let mut binding = Command::new("ffmpeg");
        let res = binding.arg("-f")
            .arg("concat")
            .arg("-safe")
            .arg("0")
            .arg("-i")
            .arg(file)
            .arg("-c")
            .arg("copy")
            .arg(target).output().unwrap().status;
        if res.success() {
            Ok(true)
        } else {
            println!("{}", res.to_string());
            Ok(false)
        }
    }

    pub fn handle_combine_ts(reg_name: String, reg_start: i32, reg_end: i32, target_name: String) -> Result<bool, Error> {
        let files = get_reg_files(reg_name.clone(), reg_start, reg_end).expect("解析失败");
        let file_name = to_files().expect("生成文件失败");
        let mut target = String::default();
        if target_name.is_empty() {
            target = format!("./{}", get_reg_file_name(reg_name.to_owned()));
        } else {
            target = format!("./{}", target_name.clone());
        }
        white_to_files(files.clone(), file_name.clone()).expect("写入文件失败");
        let res = combine_ts(file_name.clone(), target).expect("合并文件失败");
        Ok(res)
    }
}