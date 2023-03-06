use std::env;


use std::process;

use minigrep::Config;
// 重构前面对的四个问题：
// 1. 最好将函数拆分开来，让一个函数只负责一项任务。
// 2. 最好将多个配置变量合并至一个结构体内，从而让它们的用途变得更加清晰。
// 3. 目前的expect没有办法给用户提供任何有用的排错信息。
// 4. 最好将用于错误处理的代码集中放置，从而使将来的维护者在需要修改错误处理相关逻辑时只用考虑这一处代码。

// 二进制项目的一些原则：
// 将程序拆分为main.rs和lib.rs，并将实际的业务逻辑放入lib.rs。
// 当命令行解析逻辑相对简单时，将它留在main.rs中也无妨。
// 当命令行解析逻辑开始变得复杂时，同样需要将它从main.rs提取至lib.rs中。

//经过这样的拆分之后，保留在main函数中的功能应当只有：
// • 调用命令行解析的代码处理参数值。
// • 准备所有其他的配置。
// • 调用lib.rs中的run函数。
// • 处理run函数可能出现的错误。

// 这种模式正是关注点分离思想的体现：main.rs负责运行程序，而lib.rs则负责处理所有真正的业务逻辑。





fn main() {
    // 迭代器会产出一系列的值，而我们可以通过调用迭代器的collect方法来生成一个包含所有产出值的集合，比如动态数组。
    // 我们在main函数的第一行调用了env::args，并立刻使用collect函数将迭代器转换成一个包含所有迭代器产出值的动态数组。
    // 由于collect函数可以被用来创建多种不同的集合，所以我们显式地标注了args的类型来获得一个包含字符串的动态数组。
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // main函数已经不需要再关心变量和命令行参数之间的关系了。
    // let config = parse_config(&args);
    // 再次更新：把parse_config改为Config的new函数
    // let config = Config::new(&args);
    // match config {
    //     Ok(msg) => println!("{}", msg.filename),
    //     Err(err) => println!("{}", err)
    // }

    // 再次更新： 为了处理错误情形并打印出对用户友好的信息，
    // 我们需要修改main函数来处理Config::new返回的Result值，
    // 另外，我们还需要取代之前由panic! 实现的退出命令行工具并返回一个非0的错误码的功能。
    // 程序在退出时向调用者（父进程）返回非0的状态码是一种惯用的信号，它表明当前程序的退出是由某种错误状态导致的。

    // 使用unwrap_or_else可以让我们执行一些自定义的且不会产生panic! 的错误处理策略。
    // 当Result的值是Ok时，这个方法的行为与unwrap相同：它会返回Ok中的值。
    // 但是，当值为Err时，这个方法则会调用闭包（closure）中编写的代码，也就是我们定义出来并通过参数传入unwrap_or_else的这个匿名函数

    // 闭包的参数被写在两条竖线之间，而unwrap_or_else则会将Err中的值，
    // 也就是示例12-9中添加的not enough arguments，作为参数err传递给闭包❹。闭包中的代码可以在随后运行时使用参数err中的值。
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
       });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);


    // 我们使用了if let而不是unwrap_or_else来检查run的返回值，
    // 并在返回Err值的情况下调用了process::exit(1)。
    // 和Config::new返回一个Config实例不同，run函数并不会返回一个需要进行unwrap的值。
    // 因为run函数在运行成功时返回的是()，而我们只关注产生错误时的情形，所以没有必要调用unwrap_or_else把这个必定是()的值取出来。
    if let Err(e) = minigrep::run(config){
        eprintln!("error is {}", e);
        process::exit(1);
    }







    // 它会打开对应文件并使用Result<String>类型返回文件的内容
    // let contents = fs::read_to_string(filename) 
    //        .expect("Something went wrong reading the file");
   
    // println!("With text:\n{}", contents);
}


// run的初始方法，但是也有改进之处：我们不希望它直接panic，而是要给出足够多的提示信息。
// fn run(config: Config) {
//     let contents = fs::read_to_string(config.filename)
//         .expect("something went wrong reading the file");

//     println!("With text:\n{}", contents);
// }


// // run的升级版
// // Box<dyn Error>意味着函数会返回一个实现了Error trait的类型，但我们并不需要指定具体的类型是什么。
// // 这意味着我们可以在不同的错误场景下返回不同的错误类型，语句中的dyn关键字所表达的正是这种“动态”（dynamic）的含义。
// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.filename)?;

//     println!("With text:\n{}", contents);

//     Ok(())
// }


// --略--





// struct Config{
//     query: String,
//     filename: String
// }

// impl Config{
//     fn new(args: &[String]) -> Result<Config, & 'static str>{
//         // 对于参数不够的情况下，给出相关的提示
//         // 但是对于操作失误的错误类型，应当给出问题提示，而不是直接panic
//         // if args.len() < 3 {
//         //     panic!("现在的参数不足3");
//         // }

//         // 我们可以返回一个Result值，它会在成功的情况下包含Config实例，并在失败的情况下携带具体的问题描述。
//         // 当我们在main函数中调用Config::new时，就可以使用Result类型来表明当前是否存在问题。
//         // 接着，我们还可以在main函数中将可能出现的Err变体转换为一种更加友好的形式来通知用户。
//         // 使用这种方法可以避免调用panic! 时在错误提示信息前后产生thread 'main'和RUST_BACKTRACE等内部信息。

//         if args.len() < 3 {
//             return Err("not enough 参数");
//         }


//         let query = args[1].clone();
//         let filename = args[2].clone();
//         Ok(Config { query, filename })
//     }
// }

// 函数：用来解析输入的参数
// 我们还可以再稍微改进一下parse_config函数。
// 目前的函数返回了一个元组，但我们在使用时又立即将元组拆分为了独立的变量。
// 这种迹象说明当前程序中建立的抽象结构也许是不对的。
// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();

//     // 这样做确实比存储字符串的引用消耗了更多的时间和内存，
//     // 但同时也省去了管理引用的生命周期的麻烦，从而让代码更加简单直接。
//     // 在这个场景中，用少许的性能交换更多的简捷性是非常值得的取舍。
//     Config { query: query, filename: filename }
// }