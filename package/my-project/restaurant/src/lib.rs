// src/main.rs与src/lib.rs被称作单元包的根节点，因为这两个文件的内容各自组成了一个名为crate的模块

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// 1个含有其他功能模块的front_of_house模块
// 因为front_of_house模块被定义在与eat_at_restaurant相同的模块下，所以相对路径能够在eat_at_restaurant中从这个模块开始寻址。
mod front_of_house {
    // 模块内可以继续定义其他模块，如本例中的hosting与serving模块。
    // 模块内同样也可以包含其他条目的定义，比如结构体、枚举、常量、trait或如示例7-1中所示的函数。


    //路径有两种形式：
    // • 使用单元包名或字面量crate从根节点开始的绝对路径。
    // • 使用self、super或内部标识符从当前模块开始的相对路径。
    // 绝对路径与相对路径都由至少一个标识符组成，标识符之间使用双冒号（::）分隔。

    // Rust中的所有条目（函数、方法、结构体、枚举、模块及常量）默认都是私有的。
    pub mod hosting {
        pub fn add_to_waitlist() {}


        // 这一修改使我们在访问front_of_house时，可以正常访问hosting。但hosting中的内容却依旧是私有的。将模块变为公开状态并不会影响到它内部条目的状态。模块之前的pub关键字仅仅意味着祖先模块拥有了指向该模块的权限。
        fn seat_at_table() {}
    }

    // 模块不仅仅被用于组织代码，同时还定义了Rust中的私有边界（privacy boundary）：
    // 外部代码无法知晓、调用或依赖那些由私有边界封装了的实现细节。
    // 因此，当你想要将一个条目（比如函数或结构体）声明为私有时，你可以将它放置到某个模块中。

    //Rust中的所有条目（函数、方法、结构体、枚举、模块及常量）默认都是私有的。
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// 使用use将函数的父模块引入作用域意味着，我们必须在调用函数时指定这个父模块，
// 从而更清晰地表明当前函数没有被定义在当前作用域中。当然，这一方式同样也尽可能地避免了重复完整路径。

// 通过在单元包的根节点下添加use crate::front_of_house::hosting，hosting成为了该作用域下的一个有效名称，就如同hosting模块被定义在根节点下一样。
// use crate::front_of_house::hosting;
// 使用use来指定相对路径稍有一些不同。我们必须在传递给use的路径的开始处使用关键字self
use self::front_of_house::hosting;

//另一方面，当使用use将结构体、枚举和其他条目引入作用域时，我们习惯于通过指定完整路径的方式引入。
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

use std::fmt;
use std::fmt::Error;
use std::io;

fn function1() -> fmt::Result<(),Error> {
    // --略--
}

fn function2() -> io::Result<> {
    // --略--
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // hosting::seat_at_table();
}


fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        //同样也可以从父模块开始构造相对路径，这一方式需要在路径起始处使用super关键字
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // 因为back_of_house::Breakfast拥有了一个私有字段，所以这个结构体需要提供一个公共的关联函数来构造Breakfast的实例（也就是本例中的summer）
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}


pub fn eat_at_restaurant() {
    // 选择黑麦面包作为夏季早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 修改我们想要的面包类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 接下来的这一行无法通过编译，我们不能看到或更换随着食物附带的季节性水果
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}


// pub fn eat_at_restaurant() {
//     // 绝对路径
//     //add_to_waitlist函数与eat_at_restaurant被定义在相同的单元包中，所以我们可以使用crate关键字来开始一段绝对路径。
//     crate::front_of_house::hosting::add_to_waitlist();

//     // 相对路径
//     //这个路径从front_of_house开始，也就是从与eat_at_restaurant定义的模块树级别相同的那个模块名称开始。
//     // 此时的路径类似于文件系统中的front_of_house/hosting/add_to_waitlist。以名称开头意味着这个路径是相对的。
//     // 大部分的Rust开发者会更倾向于使用绝对路径，因为我们往往会彼此独立地移动代码的定义与调用代码。
//     front_of_house::hosting::add_to_waitlist();
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
