pub fn main(){
    // 除了引用，Rust还有另外一种不持有所有权的数据类型：切片（slice）。切片允许我们引用集合中某一段连续的元素序列，而不是整个集合。
    let s = String::from("hello world");
        
    let hello = &s[0..5];
    let world = &s[6..11];

    let mut ss = String::from("hello world");
    // let word = first_word(&ss);

    // 当我们拥有了某个变量的不可变引用时，我们就无法同时取得该变量的可变引用
    ss.clear();


    println!("the first word is {}", word);
    //字符串切片的边界必须位于有效的UTF-8字符边界内。尝试从一个多字节字符的中间位置创建字符串切片会导致运行时错误。


    // 字符串字面量就是切片
    let s = "Hello, world!";


    let my_string = String::from("hello world"); 
    // first_word 可以接收String对象的切片作为参数 
    let word = first_word(&my_string[..]); 
 
    let my_string_literal = "hello world"; 
 
    // first_word 可以接收字符串字面量的切片作为参数 
    let word = first_word(&my_string_literal[..]); 
 
    // 由于字符串字面量本身就是切片，所以我们可以在这里直接将它传入函数，
    // 而不需要使用额外的切片语法！ 
    let word = first_word(my_string_literal);


    // 数组切片
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
}

// 更好的调用方式
//fn first_word(s: &str) -> &str {
    // String可以看作是对str的封装
    // &str 是对 String 后备存储的直接引用，而 &String 是对“包装器”对象的引用。
    // &str 可用于子字符串，即它们是切片。 &String 总是引用整个字符串。


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

