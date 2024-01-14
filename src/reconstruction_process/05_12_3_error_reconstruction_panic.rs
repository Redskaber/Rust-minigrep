use std::{env, fs};


struct Config<'a> {
    query: &'a String,
    filename: &'a String,
}

// 12.3 重构:改进模块和错误处理
impl<'a> Config<'a> {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            // 错误处理1： 直接使用panic!: 不考虑使用对象, 报错信息杂乱，无法定位
            panic!("not enough arguments");
        }

        let query = &args[1];
        let filename = &args[2];
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
