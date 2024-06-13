use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoInfo {
    width: i32,
    height: i32,
    duration: i32,//毫秒
    video_rate: i32,
    audio_rate: i32,
    fps: f32,
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
    duration_ts: Option<i32>
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
                        video.fps = format!("{:.2}", (year.parse::<f32>().unwrap())  / (month.parse::<f32>().unwrap())).parse::<f32>().unwrap()
                    },
                    None=> {}
                }
                video.duration = i.duration_ts.unwrap()/1000;
                video.video_rate = i.bit_rate.parse::<i32>().unwrap();
            } else if i.codec_type == "audio" {
                video.audio_rate = i.bit_rate.parse::<i32>().unwrap();
            }
        }
        return video;
    }
}

pub mod cmd {
    use std::fmt::Error;
    use std::process::Command;
    use crate::cmd::{Ffprobe, VideoInfo};

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
        println!("{} file --- target {}", file.clone(), target.clone());
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

    pub fn get_video_info(file: &str) -> Option<VideoInfo> {
        let mut ffprobe = Command::new("ffprobe");
        let mut prob_result = ffprobe
            .arg("-v")
            .arg("quiet")
            .arg("-print_format")
            .arg("json")
            .arg("-show_format")
            .arg("-show_streams")
            .arg(file.to_owned())
            .output()
            .unwrap();
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