//#[macro_use]
//extern crate dump;

use std::env;
use std::process::{Command,ExitStatus};
use std::net::TcpStream;

use tokio;
#[tokio::main]
async fn main() {
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
    //dump!(cmd);

    if cmd.contains("pull") || cmd.contains("clone") {
        run_git(cmd.to_string());
    }
    
    // 判断args是否zip格式文件，如果是，则解压
    if cmd.contains(".zip") {
        run(format!("unzip {}", cmd));
    }

    // 如果包含youtube链接，则解析链接，并下载视频
    if cmd.contains("youtube.com") {
        check_install_conda("yt-dlp".to_string());
        run_conda(format!("yt-dlp {}", cmd));
    }

    // 如果包含drive.google.com链接，使用gdown下载
    if cmd.contains("drive.google.com") {
        check_install_conda("gdown".to_string());
        run_conda(format!("gdown {}", cmd));
    }
}

fn run_git(cmd: String) {
    check_install("which git".to_string(), "rpm-ostree install git".to_string());
    run(format!("git config --global http.sslVerify false"));
    run(format!("git config --global http.postBuffer 1048576000"));

    // todo 完成fastgithub 的安装
    //run(format!("/var/home/core/下载/fastgithub_linux-x64/fastgithub &"));

    tokio::spawn(async move {
        run_quite(format!("./fastgithub.sh"));
    });

    // 循环 300 次,共 30 秒
    for _ in 0..300 {
        std::thread::sleep(std::time::Duration::from_millis(100));

        if let Ok(_stream) = TcpStream::connect("127.0.0.1:38457") {
            println!("Connected to the git proxy server!");
            run(format!("git -c http.proxy=\"http://127.0.0.1:38457\" {}", cmd));
            break;
        }
    }
    
    rkill_lib::kill_process_by_pid("fastgithub".to_string()).unwrap();
}

fn run_conda(cmd: String) -> ExitStatus {
    run(format!("source ~/.miniconda/bin/activate && {}", cmd))
}

fn check_install_conda(software: String) { 
    check_install(
        "source ~/.miniconda/bin/activate && which conda".to_string(), 
        "curl -O https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh && sh Miniconda3-latest-Linux-x86_64.sh -b -p $HOME/.miniconda && rm -rf Miniconda3-latest-Linux-x86_64.sh".to_string()
    );
    check_install(
        format!("source ~/.miniconda/bin/activate && which {}", software),
        format!("source ~/.miniconda/bin/activate && pip install {}", software)
    );
}

// 检测安装软件是否符合要求
fn check_install(which: String, install_command: String) {
    if run_quite(format!("{}",which)).success() == false {
        run(format!("{}", install_command));
    }
}
    
fn run(command : String) -> ExitStatus {
    Command::new("sh").arg("-c").arg(command).status().unwrap()
}

// run quite 只适合不等待的程序
fn run_quite(command : String) -> ExitStatus {
    Command::new("sh").arg("-c").arg(command).output().unwrap().status
}
