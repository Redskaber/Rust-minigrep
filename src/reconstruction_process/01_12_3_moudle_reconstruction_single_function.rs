// 根据导包规则：导入上一级
use std::{env, fs};

// 12.3 重构:改进模块和错误处理
fn parse_config(args: &[String]) ->(&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}

fn reconstruction_single_function() {
    // 12.1 接收命令行参数: 可执行程序, arg, arg, arg...
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_config(&args);

    // 12.2 读取文件
    let context: String = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("{}", context);
}

