fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // 这个方法同样也可以直接作用于字面量：
    let s = "initial contents".to_string();

    //我们同样也可以使用函数String::from来基于字符串字面量生成String。
    let s = String::from("initial contents");

    let mut s = String::from("foo");
    // 可以使用push_str方法来向String中添加一段字符串切片
    s.push_str("bar");

    // push方法接收单个字符作为参数，并将它添加到String中。示例8-17展示了如何使用push方法向String的尾部添加字符l。
    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    //将第二个字符串的引用与第一个字符串相加
    // 编译器可以自动将&String类型的参数强制转换为&str类型。
    // 当我们调用add函数时，Rust使用了一种被称作解引用强制转换的技术，将&s2转换为了&s2[..]。
    let s3 = s1 + &s2; // 注意这里的s1已经被移动且再也不能被使用了
    println!("{}", s3);

    // 假如你需要拼接多个字符串，那么使用+运算符可能就会显得十分笨拙了：
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // 对于这种复杂一些的字符串合并，我们可以使用format! 宏：
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    //这段使用format! 的代码要更加易读，并且不会夺取任何参数的所有权。
    let s = format!("{}-{}-{}", s1, s2, s3);

    // format! 会将结果包含在一个String中返回。这段使用format! 的代码要更加易读，并且不会夺取任何参数的所有权。
    println!("{}", s1);

    let s1 = String::from("hello");
    // Rust中的字符串并不支持索引。
    // let h = s1[0];

    // String实际上是一个基于Vec<u8>的封装类型。
    let len = String::from("Hola").len(); // 4
    // 对字符串中字节的索引并不总是能对应到一个有效的Unicode标量值
    let len = String::from("Здравствуйте").len(); //24

    // 在Rust中，我们实际上可以通过3种不同的方式来看待字符串中的数据：字节、标量值和字形簇（最接近人们眼中字母的概念）。
    // 尝试通过索引引用字符串通常是一个坏主意，因为字符串索引操作应当返回的类型是不明确的
    let hello = "Здравствуйте";

    // 切片的顺序是对字节进行规约
    let s = &hello[0..4];
    println!("{}", s);

    // 调用chars会分别返回6个类型为char的值
    for c in "".chars() {
        println!("{}", c);
    }
}
