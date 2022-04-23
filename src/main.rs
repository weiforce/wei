#[macro_use]
extern crate dump;

use std::env;
use std::process::{Command,ExitStatus};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("wei https://www.youtube.com/watch?v=GQH-zWUylPY");
        return;
    }

    // 如果参数二包含youtube链接，则解析链接，并下载视频
    if args[1].contains("youtube.com") {
        let video_url = args[1].to_string();
        let command = format!("yt-dlp {}", video_url);
        run(command);
    }

    // 如果参数二包含drive.google.com链接，使用gdown下载
    if args[1].contains("drive.google.com") {
        let url = args[1].to_string();

        // 判断gdown是否安装，如果没有安装，则询问是否安装gdown
        if run(format!("which gdown")).success() == false {
            println!("gdown not installed, install it now? (y/n)");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            println!("df");
            dump!(input);
            println!("df");
            if input.trim() == "y" {
                let _ = Command::new("pip").arg("install").arg("gdown");
            } else {
                println!("Please install gdown first.");
                return;
            }
        }

        run(format!("gdown {}", url));
    }
    
}

fn run(command : String) -> ExitStatus {
     Command::new("sh").arg("-c").arg(command).status().unwrap()
}