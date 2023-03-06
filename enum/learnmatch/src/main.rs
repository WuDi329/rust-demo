// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

#[derive(Debug)] // 使我们能够打印并观察各州的设计
enum UsState {
    Alabama,
    Alaska,
    // --略--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    //首先，我们使用的match关键字后面会跟随一个表达式，也就是本例中的coin
  match coin {
    // 每个分支所关联的代码同时也是一个表达式，而这个表达式运行所得到的结果值，同时也会被作为整个match表达式的结果返回。
      Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> i32 {
    match x {
      None => 0,
      Some(i) => i + 1,
    }
}

//将match与枚举相结合在许多情形下都是非常有用的。
//你会在Rust代码中看到许多类似的套路：使用match来匹配枚举值，并将其中的值绑定到某个变量上，接着根据这个值执行相应的代码。
fn get_plus_one(x: Option<i32>) -> i32 {
    match x {
        None => 0,
        Some(i) => i+1
    }
}

// 此段代码的问题在于我们忘记了处理值是None的情形。幸运的是，这是一个Rust可以轻松捕获的问题。假如我们尝试去编译这段代码，就会看到如下所示的错误提示信息：
// 'None' not covered
// Rust中的匹配是穷尽的（exhausitive）：我们必须穷尽所有的可能性，来确保代码是合法有效的
fn plus_one_nonone(x: Option<i32>) -> Option<i32>{
    match x {
        Some(i) => Some(i + 1),

        // 有的时候，我们可能并不想要处理所有可能的值，
        // Rust同样也提供了一种模式用于处理这种需求。
        // 例如，一个u8可以合法地存储从0到255之间的所有整数。
        // 但假设我们只关心值为1、3、5或7时的情形，我们就没有必要去列出0、2、4、6、8、9直到255等其余的值。所幸我们也确实可以避免这种情形，即通过使用一个特殊的模式_来替代其余的值：
        _ => None,
    }
}

fn special1_3(x: Option<i32>) -> i32 {
    match x{
        Some(3) => 3,
        Some(1) => 1,
        _ => 0,
    }
}





fn main(){
    let region: UsState = UsState::Alabama;
    let coin = Coin::Quarter(region);
    value_in_cents(coin);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let three = Some(3);
    let one = Some(1);
    let three = special1_3(three);
    let one = special1_3(one);
    let four = Some(4);
    let four = special1_3(four);
    println!("{}, {}, {}", one, three, four);

    let six = get_plus_one(five);
    println!("{}", six);
    let zero = get_plus_one(None);
    println!("{}", zero);

    let some_u8_value = 0u8;
    match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    // 通过使用一个特殊的模式_来替代其余的值：
    // 这里的_模式可以匹配任何值。通过将它放置于其他分支后，可以使其帮我们匹配所有没有被显式指定出来的可能的情形。
    // 与它对应的代码块里只有一个()空元组，所以在_匹配下什么都不会发生。
    // 使用它也就暗示了，我们并不关心那些在_通配符前没有显式列出的情形，且不想为这些情形执行任何操作。
    _ => (),
    }


    let some_u8_value = Some(3);
    // 我们可以使用if let以一种更加简短的方式实现这段代码。
    // 使用if let意味着你可以编写更少的代码，使用更少的缩进，使用更少的模板代码。
    // 但是，你也放弃了match所附带的穷尽性检查。究竟应该使用match还是if let取决于你当时所处的环境，这是一个在代码简捷性与穷尽性检查之间取舍的过程。
    if let some_u8_value = Some(3) {
        println!("three");
    }
    // 这里的if let语法使用一对以=隔开的模式与表达式。它们所起的作用与match中的完全相同，表达式对应match中的输入，而模式则对应第一个分支。


    let mut count = 0;
    let region: UsState = UsState::Alaska;
    let coin = Coin::Quarter(region);
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }\
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

// 匹配分支另外一个有趣的地方在于它们可以绑定被匹配对象的部分值，而这也正是我们用于从枚举变体中提取值的方法。