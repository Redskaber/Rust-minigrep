use std::{env, fs};


struct Config {
    query: String,
    filename: String,
}

// 12.3 重构:改进模块和错误处理
impl Config {
    // 使用笨方法：clone 会浪费资源并且效率低
    fn new(args: &[String]) ->Config {
        let query = args[1].clone();    
        let filename = args[2].clone();
        Config {query, filename}
    }
}

fn main() {
    // 12.1 接收命令行参数: 可执行程序, arg, arg, arg...
    let args: Vec<String> = env::args().collect();
    let config:Config = Config::new(&args);

    // 12.2 读取文件
    let context: String = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");
    println!("{}", context);
}
