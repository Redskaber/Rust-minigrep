use std::{fs, env, error::Error};

#[derive(Debug)]
pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String,
    pub case_sensitive: bool,
}

// 12.3 重构:改进模块和错误处理
impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }

        let query = &args[1];
        let filename = &args[2];
        // 12.5 环境变量使用 && 大小写不敏感
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); // 默认大小写敏感: true
        Ok(Config {query, filename, case_sensitive})
    }
}

// 12.4 2.实现测试功能 查询文本
pub fn search_case_sensitive<'a>(query: &str, context: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in context.lines(){
        if line.contains(query){
            result.push(line);
        }
    }
    result
}

// 12.5 环境变量使用 && 大小写不敏感
fn search_case_insensitive<'a>(query: &str, context: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    let mut result = Vec::new();
    for line in context.lines(){
        if line.to_lowercase().contains(&query){
            result.push(line);
        }
    }
    result
}


// 重构：使用Result返回文件的错误信息
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 12.2 读取文件
    let context: String = fs::read_to_string(config.filename)?;

    // 12.5 环境变量使用 && 大小写不敏感
    let result = if config.case_sensitive {
        search_case_sensitive(&config.query, &context)
    }else{
        search_case_insensitive(&config.query, &context)
    };
    for line in result{
        println!("{}", line);
    }

    Ok(())
}


// 12.4 使用TDD（测试驱动开发）开发库功能
#[cfg(test)]
mod tests{
    use super::*;

    // 12.4 1.创建测试驱动
    #[test]
    fn case_sensitive(){
        let query = "duct";
        let context = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(&query, &context));
    }

    // 12.5 使用环境变量
    #[test]
    fn case_insensitive(){
        let query = "RuSt";
        let context = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(&query, &context));
    }
}

// main:
use std::{env, process};
use minigrep::Config;

fn main() {
    // 12.1 接收命令行参数: 可执行程序, arg, arg, arg...
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|err|{
        // 12.6 将错误消息写入标准错误而不是标准输出
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    // 对Result 进行处理
    if let Err(err) = minigrep::run(config) {
        // 12.6 将错误消息写入标准错误而不是标准输出
        eprintln!("Application error: {}", err);
    };
}


