// 将所有非main函数的代码从src/main.rs转移至src/libs.rs
// 它们包括：
// • run函数的定义
// • 相关的use语句
// • Config的定义
// • Config::new函数的定义

use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    // 最后，我们还需要检查当前设置的环境变量。
    // 因为用于处理环境变量的相关函数被放置在标准库的env模块中，
    // 所以我们需要在src/libs.rs的起始处添加use std::env;语句来将该模块引入当前作用域。
    // 接着，我们会使用env模块中的var函数来检查名为CASE_INSENSITIVE的环境变量是否存在，如示例12-23所示。

    // 这段代码创建了一个新的变量case_sensitive。
    // 为了给它赋值，我们调用了env::var函数，
    // 并将环境变量CASE_INSENSITIVE的名称作为参数传递给该函数。
    // env::var函数会返回一个Result作为结果，只有在环境变量被设置时，该结果才会是包含环境变量值的Ok变体，而在环境变量未被设置时，该结果则会是一个Err变体。
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        // 我们可以返回一个Result值，它会在成功的情况下包含Config实例，并在失败的情况下携带具体的问题描述。
       // 当我们在main函数中调用Config::new时，就可以使用Result类型来表明当前是否存在问题。
       // 接着，我们还可以在main函数中将可能出现的Err变体转换为一种更加友好的形式来通知用户。
       // 使用这种方法可以避免调用panic! 时在错误提示信息前后产生thread 'main'和RUST_BACKTRACE等内部信息。
        if args.len() < 3 {
            return Err("not enough 参数");
        }


        let query = args[1].clone();
        let filename = args[2].clone();
        // 如果CASE_INSENSITIVE环境变量被设置为了某个值，那么is_err就会返回假，
        // 也就意味着程序会进行不区分大小写的搜索。因为我们不关心环境变量的具体值，
        // 只关心其存在与否，所以我们直接使用了is_err而不是unwrap、expect或其他曾经接触过的Result的方法。
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive })
    }
}

// run的升级版
// Box<dyn Error>意味着函数会返回一个实现了Error trait的类型，但我们并不需要指定具体的类型是什么。
// 这意味着我们可以在不同的错误场景下返回不同的错误类型，语句中的dyn关键字所表达的正是这种“动态”（dynamic）的含义。
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }


    // 注意，我们同时修改了旧测试中contents的值，它新增了一行包含大写D的文本“Duct tape.”，
    // 该行文本无法在区分大小写的模式下匹配到搜索值duct。
    // 这样修改测试用例可以帮助我们确保已经实现的区分大小写的搜索功能不会遭到意外损坏。
    // 在我们编写新功能的过程中，这个测试应当是一直保持通过的。

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}





pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}




pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}