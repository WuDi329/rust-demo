// 枚举Message的变体拥有不同数量和类型的内嵌数据
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("meiguanxi");
    }
}

// 两种实现方式之间的差别在于，假如我们使用了不同的结构体，那么每个结构体都会拥有自己的类型
// 我们无法轻易定义一个能够统一处理这些类型数据的函数，而我们定义的Message枚举则不同，因为它是单独的一个类型。
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String);

struct ChangeColorMessage(i32, i32, i32);

// 由于这里的Option类型描述了一种值可能不存在的情形，
// 所以它被非常广泛地应用在各种地方。
// 将这一概念使用类型系统描述出来意味着，
// 编译器可以自动检查我们是否妥善地处理了所有应该被处理的情况。
// 使用这一功能可以避免某些在其他语言中极其常见的错误。
// enum Option<T> {
//     Some(T),
//     None,
// }


// 为什么null的设计不好？
// 实际上不是概念的问题，而是处理的问题：可能存在使用非null的方式处理了null。
// 但是Option加了一层，使得不可能直接使用Option，而是要先把其中的泛型取出来。
// 能够取出来的就一定非None，取不出来的单独处理

fn main(){
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    //为什么Option<T>的设计就比空值好呢？
    //因为Option<T>和T（这里的T可以是任意类型）是不同的类型，所以编译器不会允许我们像使用普通值一样去直接使用Option<T>的值
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    //当我们在Rust中拥有一个i8类型的值时，编译器就可以确保我们所持有的值是有效的。我们可以充满信心地去使用它而无须在使用前进行空值检查。
    //而只有当我们持有的类型是Option<i8>（或者任何可能用到的值）时，我们才必须要考虑值不存在的情况，同时编译器会迫使我们在使用值之前正确地做出处理操作。
    // 换句话说，为了使用Option<T>中可能存在的T，我们必须要将它转换为T。一般而言，这能帮助我们避免使用空值时最常见的一个问题：假设某个值存在，实际上却为空。
    // let sum = x + y;
}