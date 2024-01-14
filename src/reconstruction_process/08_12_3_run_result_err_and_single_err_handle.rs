use std::{env, fs, process, error::Error};


struct Config<'a> {
    query: &'a String,
    filename: &'a String,
}

// 12.3 重构:改进模块和错误处理
impl<'a> Config<'a> {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }

        let query = &args[1];
        let filename = &args[2];
        Ok(Config {query, filename})
    }
}

// 重构：使用Result返回文件的错误信息
fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // 12.2 读取文件
    let context: String = fs::read_to_string(config.filename)?;
    println!("{}", context);
    Ok(())
}



fn main() {
    // 12.1 接收命令行参数: 可执行程序, arg, arg, arg...
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    // 对Result 进行处理
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
    };
}
