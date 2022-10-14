use std::{env, process};

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(env::args()).unwrap_or_else(|e| {
        eprintln!("解析参数错误{:?}", e);
        process::exit(1);
    });

    println!("在文件：{}    中查找：{}", config.filename, config.query);

    if let Err(e) = run(&config) {
        eprintln!("读取文件内容错误：{}", e);
        process::exit(1);
    }
}

/*fn parse_config(args: &Vec<String>) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config { query, filename }
}*/
