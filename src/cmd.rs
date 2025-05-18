use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoInfo {
    pub width: i32,
    pub height: i32,
    pub duration: i32, //毫秒
    pub video_rate: i32,
    pub audio_rate: i32,
    pub fps: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ffprobe {
    streams: Vec<FfprobeStream>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FfprobeStream {
    codec_type: String,
    width: Option<i32>,
    height: Option<i32>,
    codec_name: String,
    channels: Option<i32>,
    avg_frame_rate: String,
    bit_rate: String,
    duration_ts: Option<i32>,
}

impl From<Ffprobe> for VideoInfo {
    fn from(a: Ffprobe) -> Self {
        let mut video = VideoInfo {
            width: 0,
            height: 0,
            duration: 0,
            video_rate: 0,
            audio_rate: 0,
            fps: 0.0,
        };

        for i in a.streams {
            if i.codec_type == "video" {
                video.width = i.width.unwrap();
                video.height = i.height.unwrap();
                let regex = Regex::new(r"(?m)(\d+)\/(\d+)").unwrap();
                let string = i.avg_frame_rate;
                match regex.captures(&string) {
                    Some(cap) => {
                        // 获取第一个匹配的日期
                        let year = cap[1].to_string();
                        let month = cap[2].to_string();
                        video.fps = format!(
                            "{:.2}",
                            (year.parse::<f32>().unwrap()) / (month.parse::<f32>().unwrap())
                        )
                            .parse::<f32>()
                            .unwrap()
                    }
                    None => {}
                }
                video.duration = i.duration_ts.unwrap() / 1000;
                video.video_rate = i.bit_rate.parse::<i32>().unwrap();
            } else if i.codec_type == "audio" {
                video.audio_rate = i.bit_rate.parse::<i32>().unwrap();
            }
        }
        return video;
    }
}

pub mod cmd {
    use crate::cmd::{Ffprobe, VideoInfo};
    use std::env;
    use std::fmt::{format, Error};
    use std::fs::{self};
    use std::path::Path;
    use std::process::{Command, Stdio};

    pub fn cut(file: String, start: u32, duration: u32, target: String) -> Result<bool, Error> {
        let mut binding = Command::new("ffmpeg");
        let res = binding
            .arg("-i")
            .arg(file)
            .arg("-ss")
            .arg(start.to_string())
            .arg("-t")
            .arg(duration.to_string())
            .arg("-c:v")
            .arg("libx264")
            .arg("-c:a")
            .arg("aac")
            .arg(target)
            .output()
            .unwrap()
            .status;
        if res.success() {
            Ok(true)
        } else {
            println!("ffmpeg 截取失败-{}", res.to_string());
            Ok(false)
        }
    }

    pub fn check_base_info_exists(folder_name: String) -> bool {
        true
    }

    pub fn clear_temp_files(folder_name: String) -> bool {
        let current_dir = env::current_dir().unwrap();
        let clear_ext = vec!["ts", "m3u8", "txt"];
        let path_str = format!("./{}", folder_name.to_owned());
        let dir_path = Path::new(path_str.as_str());
        println!("now path {}, pass dir {:?}", current_dir.as_os_str().to_str().unwrap(), dir_path);

        if !dir_path.is_dir() {
            println!("-----path: {:?} is not dir", dir_path);
            return false;
        }
        for i in clear_ext {
            for entry in fs::read_dir(dir_path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();

                if path.is_file() && path.extension().unwrap().as_encoded_bytes() == i.as_bytes() {
                    fs::remove_file(path).unwrap();
                }
            }
        }
        true
    }

    pub fn download(url: String, file_name: String) -> Result<bool, Error> {
        let mut binding = Command::new("ffmpeg");
        let res = binding
            .arg("-i")
            .arg(url.to_owned())
            .arg("-c")
            .arg("copy")
            .arg("-bsf:a")
            .arg("aac_adtstoasc")
            .arg(file_name.to_owned())
            .output()
            .unwrap()
            .status;
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
        let res = binding
            .arg("-f")
            .arg("concat")
            .arg("-i")
            .arg(file)
            .arg("-c")
            .arg("copy")
            .arg(target)
            .output()
            .unwrap()
            .status;
        if res.success() {
            Ok(true)
        } else {
            println!("ffmpeg error---{}", res.to_string());
            Ok(false)
        }
    }

    // ffmpeg -i input.mp4 -b:v <视频码率> -b:a <音频码率> -r <帧率> output.mp4
    // ffmpeg -i input.mp4 -vf "scale=1280:720" -b:v 1500k -b:a 192k -r 30 -c:v libx264 -c:a aac output.mp4
    pub fn transcode_video_to_spec_params(file: String, target: String, a_b: i32, v_b: i32, fps: i32, width: i32, height: i32) -> Result<bool, Error> {
        let mut binding = Command::new("ffmpeg");
        let res = binding
            .arg("-i")
            .arg(file)
            .arg("-vf")
            .arg(format!("scale={}:{}", width, height))
            .arg("-b:v")
            .arg(v_b.to_string())
            .arg("-b:a")
            .arg(a_b.to_string())
            .arg("-r")
            .arg(fps.to_string())
            .arg("-c:v")
            .arg("libx264".to_string())
            .arg("-c:a")
            .arg("aac".to_string())
            .arg(target)
            .output()
            .unwrap()
            .status;
        if res.success() {
            Ok(true)
        } else {
            println!("{}", res.to_string());
            Ok(false)
        }
    }

    //ffmpeg -f concat -safe 0 -i filelist.txt -c copy output.mp4
    pub fn combine_ts(file: String, target: String) -> Result<bool, Error> {
        println!("{} file --- target {}", file.clone(), target.clone());
        let mut binding = Command::new("ffmpeg");
        let res = binding
            .arg("-f")
            .arg("concat")
            .arg("-safe")
            .arg("0")
            .arg("-i")
            .arg(file)
            .arg("-c")
            .arg("copy")
            .arg(target)
            .output()
            .unwrap()
            .status;
        if res.success() {
            Ok(true)
        } else {
            println!("{}", res.to_string());
            Ok(false)
        }
    }

    pub fn check_video_validity(file_path: &str) -> Result<bool,Error> {
        let output = Command::new("ffprobe")
            .args(&["-v", "error", "-show_format", "-show_streams"])
            .arg(file_path)
            .stderr(Stdio::piped())
            .output().unwrap();

        let stderr = String::from_utf8_lossy(&output.stderr);
        Ok(stderr.trim().is_empty())
    }

    pub fn get_video_info(file: &str) -> Option<VideoInfo> {
        println!("pass file name： {}---",file);
        let mut ffprobe = Command::new("ffprobe");
        let prob_result = ffprobe
            .arg("-v")
            .arg("quiet")
            .arg("-print_format")
            .arg("json")
            .arg("-show_format")
            .arg("-show_streams")
            .arg(file.to_owned())
            .output()
            .unwrap();
        println!("ffmpeg status : {}", prob_result.status);
        if prob_result.status.success() {
            let res_data: Ffprobe =
                serde_json::from_str(String::from_utf8(prob_result.stdout).unwrap().as_str())
                    .expect("无法解析 JSON");
            let video_info: VideoInfo = res_data.into();
            Some(video_info)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::cmd::get_video_info;

    #[test]
    fn test_add() {
        let file = "https://cdn.poizon.com/du_app/2020/video/222341803_byte5570027_dur0_04e0fa415de1bd39e16dfe3b7085ddb8_1608103378948_du_android_w1088h1920.mp4";
        let data = match get_video_info(file) {
            None => {}
            Some(a) => {
                println!("{:?}", a);
            }
        };
    }
}
