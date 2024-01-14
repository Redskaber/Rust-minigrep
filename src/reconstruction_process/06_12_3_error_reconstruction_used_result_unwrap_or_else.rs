use std::{env, fs, process};


struct Config<'a> {
    query: &'a String,
    filename: &'a String,
}

// 12.3 重构:改进模块和错误处理
impl<'a> Config<'a> {
    // 错误处理2： 使用Result<Config, &'static str>, ok返回Config, err返回&'static str
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }
        
        let query = &args[1];
        let filename = &args[2];
        Ok(Config {query, filename})
    }
}

fn main() {
    // 12.1 接收命令行参数: 可执行程序, arg, arg, arg...
    let args: Vec<String> = env::args().collect();

    // 错误处理2：通过返回的Result来处理错误,使用unwrap_or_else 来自定义错误信息
    let config: Config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // 12.2 读取文件
    let context: String = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");
    println!("{}", context);
}
