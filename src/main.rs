mod combine;

use clap::{arg, Args as clapArgs, Parser, Subcommand};
use std::{env};
use crate::combine::parse::{to_files, get_reg_files, combine, get_reg_file_name, white_to_files};

#[derive(Parser)]
#[command(name = "ffmpeg-tool-rs")]
#[command(author = "zmisgod", version = env ! ("CARGO_PKG_VERSION"), about = "", long_about = None,)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 合并视频
    Combine(CombineArgs),
    // /// 下载视频
    // Download(DownloadArgs),
}

#[derive(clapArgs)]
pub struct CombineArgs {
    /* 完整的视频url，格式如下
        -f https://zmis.me/video0.mp4 -f https://zmis.me/video1.mp4 -f https://zmis.me/video2.mp4
     */
    #[arg(short = 'f', long = "files")]
    files: Vec<String>,

    /* 如果很多，可以放在文件中，格式如下
      https://zmis.me/video0.mp4
      https://zmis.me/video1.mp4
      https://zmis.me/video2.mp4
     */
    #[arg(short = 'l', long = "local-files", default_value_t = String::from(""))]
    local: String,

    // 正则模式， 输入文件 https://zmis.me/video(.*).mp4
    #[arg(short = 'r', long = "reg-name", default_value_t = String::from(""))]
    reg_name: String,

    // 正则模式，输入的文件开始数字
    #[arg(long = "reg-file-start")]
    reg_name_start: i32,

    // 正则模式，输入的文件结束数字
    #[arg(long = "reg-file-end")]
    reg_name_end: i32,

    // 输出的文件目录
    #[arg(long = "target_folder", default_value_t = String::from(""))]
    target_folder: String,

    // 输出的文件名
    #[arg(long = "target_file_name", default_value_t = String::from(""))]
    target_file_name: String,
}

#[derive(clapArgs)]
pub struct DownloadArgs {}

#[actix_web::main]
pub async fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Combine(args) => {
            for value in args.files {
                println!("{}", value);
            }
            let files = get_reg_files(args.reg_name.clone(), args.reg_name_start, args.reg_name_end).expect("解析失败");
            let file_name = to_files().expect("生成文件失败");
            println!("file name = {}", file_name.to_owned());
            let mut target = String::default();
            if args.target_file_name.is_empty() {
                target = format!("./{}", get_reg_file_name(args.reg_name.to_owned()));
            } else {
                if !args.target_folder.is_empty() {
                    target = format!("{}/{}", args.target_folder, args.target_file_name);
                } else {
                    target = format!("./{}", args.target_file_name);
                }
            }
            white_to_files(files.clone(), file_name.clone()).expect("写入文件失败");

            println!("{}", target);
            let res = combine(file_name.clone(), target).expect("合并文件失败");
            println!("res {}", res);
        }
    }
}