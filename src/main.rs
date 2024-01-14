// This project is impl minigrep in rust.
// ps: 
//   cargo run xxx xxx.txt 
//   xxx is'not in xxx.txt
// 
// 实现步骤：
// 12.1 接收命令行参数
// 12.2 读取文件
// 12.3 重构:改进模块和错误处理: 
//  - 模块重构：整体 -> 单一职责(函数封装) -> 关联性与可维护性(结构体封装) -> 性能优化(引用传递|移动语义|生命周期)
//  - 错误处理：通过返回的Result来处理错误，使用unwrap_or_else 来自定义错误信息
//  - 模块重构：lib模块中放置错误处理逻辑，main模块中放置简单的调用逻辑
// 12.4 使用TDD（测试驱动开发）开发库功能
//  - 测试驱动开发：TDD 是一种设计模式，它的核心是测试（测试用例）。
//      - 步骤：编写测试（失败并按照预期结果） -> 编写代码（成功并按照预期结果） -> 重构代码（成功并按照预期结果）
// 12.5 使用环境变量
// 12.6 将错误消息写入标准错误而不是标准输出


// use std::env::args();
// use std::env::args_os();
// use std::fs::read_to_string();
// 根据导包规则：导入上一级
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

