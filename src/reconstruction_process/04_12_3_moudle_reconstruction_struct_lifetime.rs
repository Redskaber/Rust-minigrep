use std::{env, fs};


struct Config<'a> {
    query: &'a String,
    filename: &'a String,
}

// 12.3 重构:改进模块和错误处理
impl<'a> Config<'a> {
    // 根据生命周期规则2：当输入参数只有一个时，参数与返回值的生命周期相同，不需要指定
    fn new(args: &[String]) -> Config {
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

