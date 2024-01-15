use std::{fs, env, error::Error};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

// 12.3 重构:改进模块和错误处理
impl Config{
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }

        args.next();
        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        // 12.5 环境变量使用 && 大小写不敏感
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); // 默认大小写敏感: true
        Ok(Config {query, filename, case_sensitive})
    }
}

// 12.4 2.实现测试功能 查询文本
pub fn search_case_sensitive<'a>(query: &str, context: &'a str) -> Vec<&'a str>{
    context.lines()
        .into_iter()
        .filter(|line| line.contains(query))
        .collect()
}

// 12.5 环境变量使用 && 大小写不敏感
fn search_case_insensitive<'a>(query: &str, context: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); // 转换为小写, 新对象
    context.lines()
        .into_iter()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}


// 重构：使用Result返回文件的错误信息
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
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
