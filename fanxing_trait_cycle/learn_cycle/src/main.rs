fn main() {
    // 类似地，当引用的生命周期可能以不同的方式相互关联时，
    // 我们就必须手动标注生命周期。
    // Rust需要我们注明泛型生命周期参数之间的关系，
    // 来确保运行时实际使用的引用一定是有效的。
    println!("Hello, world!");

    // 生命周期最主要的目标在于避免悬垂引用，进而避免程序引用到非预期的数据。

    // {
    // let r;

    // {
    //     // `x` does not live long enough, borrowed value does not live long enough
    //     // 在编译过程中，Rust会比较两段生命周期的大小，并发现r拥有生命周期'a，但却指向了拥有生命周期'b的内存。
    //     // 这段程序会由于'b比'a短而被拒绝通过编译：被引用对象的存在范围短于引用
    // let x = 5;
    // r = &x;
    // }

    // println!("r: {}", r);
    // }

    // Rust编译器拥有一个借用检查器（borrow checker），它被用于比较不同的作用域并确定所有借用的合法性。


    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);



    // 在这个示例中，string1直到外部作用域结束都会是有效的，而string2的有效性则只持续到内部作用域结束的地方。
    // 运行这段代码，它可以正常地通过借用检查器进行编译，并最终输出The longest string is long string is long。
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }


        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
        }
        println!("The longest string is {}", result);
    

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
            .next()
            .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };

    // Rust中还存在一种特殊的生命周期'static，它表示整个程序的执行期。所有的字符串字面量都拥有'static生命周期，我们可以像下面一样显式地把它们标注出来：
    let s: &'static str = "I have a static lifetime.";
    // 字符串的文本被直接存储在二进制程序中，并总是可用的。因此，所有字符串字面量的生命周期都是'static。


}

// 它不知道x与y的生命周期是如何与返回值的生命周期相关联的。
// 为了解决这个问题，我们会添加一个泛型生命周期参数，并用它来定义引用之间的关系，进而使借用检查器可以正常地进行分析。
// this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`

// 我们会将生命周期参数的标注填写在&引用运算符之后，并通过一个空格符来将标注与引用类型区分开来。
// 一个指向i32且不带生命周期参数的引用 &i32
// 一个指向i32且带有名为'a的生命周期参数的引用 &'a i32
// 以及一个同样拥有生命周期'a的指向i32的可变引用。&'a mut i32

// 例如，假设我们编写了一个函数，这个函数的参数first是一个指向i32的引用，并且拥有生命周期'a。
// 它的另一个参数second同样也是指向i32且拥有生命周期'a的引用。
// 这样的标注就意味着：first和second的引用必须与这里的泛型生命周期存活一样长的时间。

// 在这个签名中我们所表达的意思是：参数与返回值中的所有引用都必须拥有相同的生命周期。我们将这个生命周期命名为'a并将它添加至每个引用中
// 这段代码的函数签名向Rust表明，函数所获取的两个字符串切片参数的存活时间，必须不短于给定的生命周期'a。
// 注意，longest函数本身并不需要知道x与y的具体存活时长，只要某些作用域可以被用来替换'a并满足约束就可以了。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 当我们将具体的引用传入longest时，被用于替代'a的具体生命周期就是作用域x与作用域y重叠的那一部分。
    // 换句话说，泛型生命周期'a会被具体化为x与y两者中生命周期较短的那一个。因为我们将返回的引用也标记为了生命周期参数'a，所以返回的引用在具化后的生命周期范围内都是有效的。
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


// 当函数返回一个引用时，返回类型的生命周期参数必须要与其中一个参数的生命周期参数相匹配。
// 当返回的引用没有指向任何参数时，那么它只可能是指向了一个创建于函数内部的值，
// 由于这个值会因为函数的结束而离开作用域，所以返回的内容也就变成了悬垂引用。


fn longest_err<'a>(x: &str, y: &str) -> String {
    let result = String::from("really long string");
    result
}
// 从根本上说，生命周期语法就是用来关联一个函数中不同参数及返回值的生命周期的。
// 一旦它们形成了某种联系，Rust就获得了足够的信息来支持保障内存安全的操作，并阻止那些可能会导致悬垂指针或其他违反内存安全的行为。

// 实际上，我们也可以在结构体中存储引用，不过需要为结构体定义中的每一个引用都添加生命周期标注。
// 这个标注意味着ImportantExcerpt实例的存活时间不能超过存储在part字段中的引用的存活时间。
struct ImportantExcerpt<'a> {
     part: &'a str,
   }

// 当我们需要为某个拥有生命周期的结构体实现方法时，可以使用与示例10-11中展示的与泛型参数相似的语法。
// 声明和使用生命周期参数的位置取决于它们是与结构体字段相关，还是与方法参数、返回值相关。
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// 在没有显式标注的情况下，编译器目前使用了3种规则来计算引用的生命周期。
// 第一条规则作用于输入生命周期，第二条和第三条规则作用于输出生命周期。
// 当编译器检查完这3条规则后仍有无法计算出生命周期的引用时，编译器就会停止运行并抛出错误。
// 这些规则不但对fn定义生效，也对impl代码块生效。
   
// 第一条规则是，每一个引用参数都会拥有自己的生命周期参数。
// 第二条规则是，当只存在一个输入生命周期参数时，这个生命周期会被赋予给所有输出生命周期参数
// 第三条规则是，当拥有多个输入生命周期参数，而其中一个是&self或&mut self时，self的生命周期会被赋予给所有的输出生命周期参数。
// 这条规则使方法更加易于阅读和编写，因为它省略了一些不必要的符号。

