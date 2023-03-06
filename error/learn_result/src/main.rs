// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::error::Error;
use std::fs::File;
use std::io::ErrorKind;


//这里的Box<dyn Error>被称作trait对象，我们将在第17章讨论它。
// 现在，你可以简单地将Box<dyn Error>理解为“任何可能的错误类型”。在拥有这种返回类型的main函数中使用？运算符是合法的。
fn main() -> Result<(), Box<dyn Error>>{
    println!("Hello, world!");

    // Result枚举定义了两个变体—Ok和Err
    // T代表了Ok变体中包含的值类型，该变体中的值会在执行成功时返回；
    // 而E则代表了Err变体中包含的错误类型，该变体中的值会在执行失败时返回。

    let f = File::open("hello.txt");

    // 这里的泛型参数T被替换为了成功值的类型std::fs::File，也就是文件的句柄，
    // 而错误值所对应的类型E则被替换为了std::io::Error。
    // 当File::open函数运行成功时，变量f中的值将会是一个包含了文件句柄的Ok实例。
    // 当它运行失败时，变量f中的值则会是一个包含了用于描述错误种类信息的Err实例。

    // let f = match f {
    //         //我们的代码告诉Rust，当结果是Ok的时候，将Ok变体内部的file值移出，并将这个文件句柄重新绑定至变量f。
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         // 这个结构体拥有一个被称作kind的方法，我们可以通过调用它来获得io::ErrorKind值。
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("tried to create file but there was a problem: {:?}", e),
    //         },
    //         other_error => panic!("there was a problem opening the file: {:?}", other_error),
    //     },
    // };

    //Result<T,E>通过使用match表达式实现了许多接收闭包的方法；我们会在第13章开始学习闭包
    // 虽然这段代码与示例9-5拥有完全一致的行为，但它却没有使用任何的match表达式，并且更为清晰易读。
    // let f = File::open("hello.txt").map_err(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Tried to create file but there was a problem: {:?}", error);
    //         })
    //     } else {
    //         panic!("There was a problem opening the file: {:?}", error);
    //     }
    // });

    // 其中一个被称为unwrap的方法实现了我们在示例9-4中编写的match表达式的效果。
    // 当Result的返回值是Ok变体时，unwrap就会返回Ok内部的值。
    // 而当Result的返回值是Err变体时，unwrap则会替我们调用panic! 宏。下面是一个在实际代码中使用unwrap的例子：
    // 在示例代码中，大家能够约定俗成地将unwrap之类可能会导致panic的方法理解为某种占位符，用来标明那些需要由应用程序进一步处理的错误，根据上下文环境的不同，具体的处理方法也会不同。
    // unwrap可以用来写demo
    // let f = File::open("hello.txt").unwrap();

    // 还有另外一个被称作expect的方法，它允许我们在unwrap的基础上指定panic! 所附带的错误提示信息。
    // 使用expect并附带上一段清晰的错误提示信息可以阐明你的意图，并使你更容易追踪到panic的起源。
    // 下面演示了expect的使用语法：

    // let f = File::open("hello.txt").expect("Failed to open hello.txt");



    // 当你编写的函数中包含了一些可能会执行失败的调用时，
    // 除了可以在函数中处理这个错误，还可以将这个错误返回给调用者，让他们决定应该如何做进一步处理。
    // 这个过程也被称作传播错误，在调用代码时它给了用户更多的控制能力。
    // 与编写代码时的上下文环境相比，调用者可能会拥有更多的信息和逻辑来决定应该如何处理错误。



        // fn read_username_from_file() -> Result<String, io::Error> ❶ {
        //  ❷ let f = File::open("hello.txt");

        //  ❸ let mut f = match f {
        //         Ok(file) => file,
        //         Err(e) => return Err(e),
        //     };

        //  ❹ let mut s = String::new();

        //  ❺ match f.read_to_string(&mut s)❻ {
        //         Ok(_) => Ok(s)❼,
        //         Err(e) => Err(e)❽,
        //     }
        // }

    //传播错误的模式在Rust编程中非常常见，所以Rust专门提供了一个问号运算符（?）来简化它的语法。

    // 通过将？放置于Result值之后，
    // 我们实现了与上面来处理Result时一样的功能。
    // 假如这个Result的值是Ok，那么包含在Ok中的值就会作为这个表达式的结果返回并继续执行程序。
    // 假如值是Err，那么这个值就会作为整个程序的结果返回，如同使用了return一样将错误传播给调用者。

    // 当？运算符调用from函数时，它就开始尝试将传入的错误类型转换为当前函数的返回错误类型。
    // 当一个函数拥有不同的失败原因，却使用了统一的错误返回类型来同时进行表达时，这个功能会十分有用。
    // 只要每个错误类型都实现了转换为返回错误类型的from函数，?运算符就会自动帮我们处理所有的转换过程。


    // 因为？运算符的功能类似于示例9-6中定义的match表达式，所以它只能被用于那些拥有Result返回类型的函数。
    // 在match表达式中，return Err(e)部分产生的返回类型是Result，所以函数的返回类型也必须是Result，才能与此处的return兼容。

    let f = File::open("hello.txt")?;





    // 当你拥有某些逻辑可以确保Result是一个Ok值时，调用unwrap也是非常合理的
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1".parse().unwrap();



    Ok(())
    
}

use std::io;
use std::io::Read;
// use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error>  {
  let f = File::open("hello.txt");


  // 首先，让我们将注意力放到这个函数的返回类型上：Result<String, io::Error>。
  // 它意味着这个函数的返回值的类型为Result<T, E>，其中的泛型参数T被替换为具体的String类型，而泛型E则被替换为具体的io::Error类型
  let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

  let mut s = String::new();

    // 假如read_to_string运行失败，我们就可以像之前处理File::open时一样，将这个错误值作为结果返回。
    // 但需要注意的是，由于这里是函数的最后一个表达式，所以我们不再需要显式地添加return。

  match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}



fn read_username_from_file_() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    // 假如这个Result的值是Ok，那么包含在Ok中的值就会作为这个表达式的结果返回并继续执行程序。
    // 假如值是Err，那么这个值就会作为整个程序的结果返回，如同使用了return一样将错误传播给调用者。

    // 被？运算符所接收的错误值会隐式地被from函数处理，这个函数定义于标准库的From trait中，用于在错误类型之间进行转换。
    // 当？运算符调用from函数时，它就开始尝试将传入的错误类型转换为当前函数的返回错误类型。
    // 当一个函数拥有不同的失败原因，却使用了统一的错误返回类型来同时进行表达时，这个功能会十分有用。
    // 只要每个错误类型都实现了转换为返回错误类型的from函数，?运算符就会自动帮我们处理所有的转换过程。
    f.read_to_string(&mut s)?;
    Ok(s)
}


fn read_username_from_file_simple() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// 因为？运算符的功能类似于示例9-6中定义的match表达式，所以它只能被用于那些拥有Result返回类型的函数。
// 在match表达式中，return Err(e)部分产生的返回类型是Result，所以函数的返回类型也必须是Result，才能与此处的return兼容


