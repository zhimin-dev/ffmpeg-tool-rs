mod combine;
mod download;
mod common;
mod m3u8;
mod cmd;

use clap::{arg, Args as clapArgs, Parser, Subcommand};
use std::{env};
use std::path::Path;
use crate::cmd::cmd::{combine, cut, download};
use crate::combine::parse::{to_files, get_reg_files, get_reg_file_name, white_to_files, combine_video};
use crate::common::now;
use crate::download::download::{get_file_name, fast_download, create_folder};
use crate::m3u8::m3u8::{get_method_from_regex, get_uri_from_regex};

#[derive(Parser)]
#[command(name = "ffmpeg-tool-rs")]
#[command(
    author = "zmisgod", version = env ! ("CARGO_PKG_VERSION"), about = "", long_about = None,
)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 合并视频
    Combine(CombineArgs),
    /// 下载视频
    Download(DownloadArgs),
    /// 截取视频
    Cut(CutArgs),
}

#[derive(clapArgs)]
pub struct CutArgs {
    /// 需要截取的视频
    #[arg(short = 'i', long = "input")]
    input: String,

    /// 视频开始的秒数
    #[arg(short = 's', long = "start", default_value_t = 0)]
    start: u32,

    /// 截取视频的时长
    #[arg(short = 'd', long = "duration", default_value_t = 3)]
    duration: u32,

    /// 输出的文件名
    #[arg(long = "target_file_name", default_value_t = String::from(""))]
    target_file_name: String,
}

#[derive(clapArgs)]
pub struct CombineArgs {
    /// 正则模式， 输入文件 https://zmis.me/video(.*).mp4
    #[arg(short = 'r', long = "reg-name", default_value_t = String::from(""))]
    reg_name: String,

    /// 正则模式，输入的文件开始数字
    #[arg(long = "reg-file-start")]
    reg_name_start: i32,

    /// 正则模式，输入的文件结束数字
    #[arg(long = "reg-file-end")]
    reg_name_end: i32,

    /// 输出的文件名
    #[arg(long = "target_file_name", default_value_t = String::from(""))]
    target_file_name: String,

    /// 相同视频参数，如果指定，则根据指定视频的视频码率、音频码率、fps进行合并, 则后面set开头的参数均被覆盖
    #[arg(long = "same_param_index", default_value_t = - 1)]
    same_param_index: i32,

    /// 指定fps
    #[arg(long = "set_fps", default_value_t = 0)]
    set_fps: i32,

    /// 指定音频码率, audio bitrate
    #[arg(long = "set_a_b", default_value_t = 0)]
    set_a_b: i32,

    /// 指定视频码率, video bitrate
    #[arg(long = "set_v_b", default_value_t = 0)]
    set_v_b: i32,
}

#[derive(clapArgs)]
pub struct DownloadArgs {
    /// m3u8链接地址
    #[arg(short = 'u', long = "url")]
    url: String,

    /// 设置为true则会优化加速下载
    #[arg(long = "fast")]
    fast: bool,

    /// 输出的文件名
    #[arg(long = "target_file_name", default_value_t = String::from(""))]
    target_file_name: String,

    /// 保存的文件夹
    #[arg(long = "folder", default_value_t = String::from(""))]
    folder: String,

    /// 下载并发数
    #[arg(long = "concurrent", default_value_t = 3)]
    concurrent: i32,
}

#[actix_web::main]
pub async fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Combine(args) => {
            let files = get_reg_files(args.reg_name.clone(), args.reg_name_start, args.reg_name_end).expect("解析失败");
            let file_name = to_files().expect("生成文件失败");
            let mut target = String::default();
            if args.target_file_name.is_empty() {
                target = format!("./{}", get_reg_file_name(args.reg_name.to_owned()));
            } else {
                target = format!("./{}", args.target_file_name);
            }
            let res = combine_video(files, file_name.clone(), target, args.same_param_index, args.set_a_b, args.set_v_b, args.set_fps).expect("合并文件失败");
            if res {
                println!("合并文件成功")
            } else {
                println!("合并文件失败")
            }
        }
        Commands::Cut(args) => {
            if args.duration <= 0 {
                println!("duration 需要 > 0");
                return;
            }
            let mut target = String::default();
            if args.target_file_name.is_empty() {
                target = format!("./{}.mp4", now());
            } else {
                target = format!("./{}", args.target_file_name);
            }
            let res = cut(args.input.clone(), args.start, args.duration, target).expect("处理失败");
            if res {
                println!("截取视频成功")
            } else {
                println!("截取视频失败")
            }
        }
        Commands::Download(args) => {
            if args.url.is_empty() {
                println!("url is required!");
                return;
            }
            let file_name = get_file_name(args.target_file_name.to_owned());
            println!("download file name: {}", file_name.clone());
            let mut res = false;
            if args.fast {
                println!("now is fast mod");
                let mut folder_name = args.folder.clone();
                if folder_name.is_empty() {
                    folder_name = format!("{}", now());
                }
                match create_folder(folder_name.clone()) {
                    Ok(dir_status) => {
                        if dir_status {
                            if !args.folder.is_empty() {
                                if let Err(e) = env::set_current_dir(&Path::new(&args.folder)) {
                                    println!("进入文件夹失败");
                                    return;
                                } else {
                                    res = fast_download(args.url, file_name, folder_name.clone(), args.concurrent).await.expect("下载失败");
                                }
                            } else {
                                res = fast_download(args.url, file_name, folder_name.clone(), args.concurrent).await.expect("下载失败");
                            }
                        } else {
                            println!("创建文件夹失败");
                            return;
                        }
                    }
                    _ => {
                        println!("出错");
                        return;
                    }
                }
            } else {
                res = download(args.url, file_name).expect("下载失败");
            }
            if res {
                println!("下载成功")
            } else {
                println!("下载失败")
            }
        }
    }
}