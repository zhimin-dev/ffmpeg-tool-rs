pub mod cmd {
    use std::fmt::Error;
    use std::process::Command;

    pub fn cut(file: String, start: u32, duration: u32, target: String) -> Result<bool, Error> {
        let mut binding = Command::new("ffmpeg");
        let res = binding.arg("-i")
            .arg(file)
            .arg("-ss")
            .arg(start.to_string())
            .arg("-t")
            .arg(duration.to_string())
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

    pub fn download(url: String, file_name: String) -> Result<bool, Error> {
        let mut binding = Command::new("ffmpeg");
        let res = binding.arg("-i")
            .arg(url.to_owned())
            .arg("-c")
            .arg("copy")
            .arg("-bsf:a")
            .arg("aac_adtstoasc")
            .arg(file_name.to_owned()).output().unwrap().status;
        if res.success() {
            Ok(true)
        } else {
            println!("{}", res.to_string());
            Ok(false)
        }
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
    pub fn combine_ts(file: String, target: String) -> Result<bool, Error> {
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
}