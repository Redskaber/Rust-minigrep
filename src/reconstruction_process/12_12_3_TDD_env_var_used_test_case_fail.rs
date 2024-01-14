use std::{fs, error::Error};


pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String,
}

// 12.3 重构:改进模块和错误处理
impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }

        let query = &args[1];
        let filename = &args[2];
        Ok(Config {query, filename})
    }
}

// 12.4 2.实现测试功能 查询文本
pub fn search_case_sensitive<'a>(query: &str, context: &'a str) -> Vec<&'a str>{
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
    vec![]
}


// 重构：使用Result返回文件的错误信息
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // 12.2 读取文件
    let context: String = fs::read_to_string(config.filename)?;
    for line in search_case_sensitive(&config.query, &context){
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
        let query = "RuCt";
        let context = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(&query, &context));
    }
}