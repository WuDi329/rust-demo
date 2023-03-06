use std::fmt::Display;

pub trait Summary {
    // 我们还可以在默认实现中调用相同trait中的其他方法，哪怕这些方法没有默认实现。
    // 基于这一规则，trait可以在只需要实现一小部分方法的前提下，提供许多有用的功能。
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("read more from {}...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 为类型实现trait与实现普通方法的步骤十分类似。
// 它们的区别在于我们必须在impl关键字后提供我们想要实现的trait名，并紧接for关键字及当前的类型名。
// 在impl代码块中，我们同样需要填入trait中的方法签名。
// 但在每个签名的结尾不再使用分号，而是使用花括号并在其中编写函数体来为这个特定类型实现该trait的方法所应具有的行为。
// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}



impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

pub struct NewsArticleB {
    pub username: String,
    pub content: String,
}

// impl Trait更适用于短小的示例，而trait约束则更适用于复杂情形。
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 
pub fn notify_double(item1: impl Summary, item2: impl Summary) {
}

// 完整写法
pub fn notify_long<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_long_double<T: Summary>(item: T){

}

// 通过+语法来指定多个trait约束假如notify函数需要在调用summarize方法的同时显示格式化后的item，
// 那么item就必须实现两个不同的trait：Summary和Display。我们可以使用+语法做到这一点：
pub fn notify_complex(item: impl Summary + Display) {
}

pub fn notify_complex_long<T: Summary + Display>(item: T) {
}


//使用where从句来简化trait约束
fn some_function_where<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    return 0;
}

// impl Summary for NewsArticleB {}