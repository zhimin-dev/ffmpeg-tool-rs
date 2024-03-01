pub mod cut {
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
}