pub mod parse {
    use std::fmt::{Error, format};
    use std::process::Command;
    use std::time::{SystemTime, UNIX_EPOCH};
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::prelude::*;

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

    fn now() -> u64 {
        let now = SystemTime::now();
        return now.duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
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
}