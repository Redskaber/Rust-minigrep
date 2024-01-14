// 12.4 使用TDD（测试驱动开发）开发库功能
//  - 测试驱动开发：TDD 是一种设计模式，它的核心是测试（测试用例）。
//      - 步骤：编写测试（失败并按照预期结果） -> 编写代码（成功并按照预期结果） -> 重构代码（成功并按照预期结果）
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

// 查询文本
pub fn search<'a>(query: &str, context: &'a str) -> Vec<&'a str>{
    vec![]  // fail
}

// 重构：使用Result返回文件的错误信息
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // 12.2 读取文件
    let context: String = fs::read_to_string(config.filename)?;
    println!("{}", context);
    Ok(())
}


// 12.4 使用TDD（测试驱动开发）开发库功能
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let context = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(&query, &context));
    }
}