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

    // 拼接args成一个字符串，忽略第一个元素
    let cmd = args.iter().skip(1).fold(String::new(), |mut acc, x| {
        acc.push_str(&x);
        acc.push_str(" ");
        acc
    });
    let cmd = cmd.trim();
    dump!(cmd);

    // 如果参数二包含youtube链接，则解析链接，并下载视频
    if cmd.contains("youtube.com") {// curl -O https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh > a.sh && 
        check_install_conda("yt-dlp".to_string());
        run_conda(format!("yt-dlp {}", cmd));
    }

    // 如果参数二包含drive.google.com链接，使用gdown下载
    if cmd.contains("drive.google.com") {
        check_install_conda("gdown".to_string());
        run_conda(format!("gdown {}", cmd));
    }
    
}

fn run_conda(cmd: String) -> ExitStatus {
    run(format!("source ~/.miniconda/bin/activate && {}", cmd))
}

fn check_install_conda(software: String) {
    check_install("source ~/.miniconda/bin/activate && which conda".to_string(), "sh a.sh -b -p $HOME/.miniconda".to_string());
    check_install(
        format!("source ~/.miniconda/bin/activate && which {}", software),
        format!("source ~/.miniconda/bin/activate && pip install {}", software)
    );
}

// 检测安装软件是否符合要求
fn check_install(which: String, install_command: String) {
    // 判断gdown是否安装，如果没有安装，则询问是否安装
    if run_quite(format!("{}",which)).success() == false {
        // println!("{} not installed, install it now? (y/n)", software);
        // let mut input = String::new();
        // std::io::stdin().read_line(&mut input).expect("Failed to read line");
        // // 如果input包含y，则安装gdown
        // if input.contains("y") {
        run(format!("{}", install_command));
        // } else {
        //     println!("Please install {} first.", software);
        //     // 程序退出
        //     std::process::exit(1);
        // }
    }
}
    

fn run(command : String) -> ExitStatus {
    Command::new("sh").arg("-c").arg(command).status().unwrap()
}

fn run_quite(command : String) -> ExitStatus {
    Command::new("sh").arg("-c").arg(command).output().unwrap().status
}
