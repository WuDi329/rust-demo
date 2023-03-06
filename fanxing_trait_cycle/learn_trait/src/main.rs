use learn_trait::*;
fn main() {
    // trait（特征）被用来向Rust编译器描述某些特定类型拥有的且能够被其他类型共享的功能，
    // 它使我们可以以一种抽象的方式来定义共享行为。
    // 我们还可以使用trait约束来将泛型参数指定为实现了某些特定行为的类型。


    // 类型的行为由该类型本身可供调用的方法组成。
    // 当我们可以在不同的类型上调用相同的方法时，我们就称这些类型共享了相同的行为。
    // trait提供了一种将特定方法签名组合起来的途径，它定义了为达成某种目的所必需的行为集合。
    println!("Hello, world!");

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    // 可以在默认实现中调用重载方法的实现
    // 不可以在重载方法中调用默认方法的实现
    println!("1 new tweet: {}", tweet.summarize());

    // 只有当trait或类型定义于我们的库中时，我们才能为该类型实现对应的trait
    // 我们可以为自定义类型，比如Tweet，实现标准库中的Display trait。能这么做的原因在于，类型Tweet定义在我们的aggregator库中。
    // 同样地，因为Summary trait也定义在我们的aggregator库中，所以也可以在aggregator库中为Vec<T>实现Summary trait。

    // 但是，我们不能为外部类型实现外部trait。
    // 例如，我们不能在aggregator库内为Vec<T>实现Display trait，因为Display与Vec<T>都被定义在标准库中，而没有定义在aggregator库中。
    // 这个限制被称为孤儿规则（orphan rule），之所以这么命名是因为它的父类型没有定义在当前库中。这一规则也是程序一致性（coherence）的组成部分，它确保了其他人所编写的内容不会破坏到你的代码，反之亦然。
    //如果没有这条规则，那么两个库可以分别对相同的类型实现相同的trait，Rust将无法确定应该使用哪一个版本。

    // let a = NewsArticleB{
    //     username: String::from("lalalalal"),
    //     content: String::from("lalallalal"),
    // };
    // println!("newsarticleb: {}", a.summarize());
}


// 通过在带有泛型参数的impl代码块中使用trait约束，我们可以单独为实现了指定trait的类型编写方法。

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 我们同样可以为实现了某个trait的类型有条件地实现另一个trait。对满足trait约束的所有类型实现trait也被称作覆盖实现
impl<T: Display> ToString for T {
    // --略--
}

// 借助于trait和trait约束，我们可以在使用泛型参数来消除重复代码的同时，
// 向编译器指明自己希望泛型拥有的功能。
// 而编译器则可以利用这些trait约束信息来确保代码中使用的具体类型提供了正确的行为。

// Rust将这些错误出现的时期转移到了编译期，并迫使我们在运行代码之前修复问题。
// 我们无须编写那些用于在运行时检查行为的代码，因为这些工作已经在编译期完成了。这一机制在保留泛型灵活性的同时提升了代码的性能。