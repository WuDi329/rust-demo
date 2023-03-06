fn main() {
    println!("Hello, world!");


    // 动态数组允许你在单个数据结构中存储多个相同类型的值，这些值会彼此相邻地排布在内存中
    // 调用函数Vec::new来创建一个空动态数组
    let v: Vec<i32> = Vec::new();

    // 使用初始值去创建动态数组
    let v = vec![1, 2, 3];


    // 在创建动态数组后将元素添加至其中
    let mut v = Vec::new();
    // 动态数组中的所有内容都会随着动态数组的销毁而销毁，其持有的整数将被自动清理干净。
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);


    let v = vec![1, 2, 3, 4, 5];

    //使用&与[]会直接返回元素的引用；
    // 首先，我们使用索引值2获得的是第三个值：动态数组使用数字进行索引，索引值从零开始。
    // 其次，使用&与[]会直接返回元素的引用；而接收索引作为参数的get方法则会返回一个Option<&T>。
    let third: &i32 = &v[2];

    println!("The third element is {}", third);

    let does_not_exist = &v[100];
    println!("{}", does_not_exist);
    // 而接收索引作为参数的get方法则会返回一个Option<&T>。
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    //[]方法会因为索引指向了不存在的元素而导致程序触发panic。
    // 假如你希望在尝试越界访问元素时使程序直接崩溃，那么这个方法就再适合不过了。
    // let does_not_exist = &v[100];
    // get方法会在检测到索引越界时简单地返回None，而不是使程序直接崩溃。
    // 当偶尔越界访问动态数组中的元素是一个正常行为时，你就应该使用这个方法。
    let does_not_exist = v.get(100);
    match does_not_exist{
        Some(num) => println!("{}", num),
        None => println!("none"),
    }

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // 动态数组中的元素是连续存储的，插入新的元素后也许会没有足够多的空间将所有元素依次相邻地放下，这就需要分配新的内存空间，并将旧的元素移动到新的空间上。
    // v.push(6);
    println!("The first element is: {}", first);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // 为了使用+=运算符来修改可变引用指向的值，
        // 我们首先需要使用解引用运算符（*）来获得i绑定的值。我们会在第15章的“使用解引用运算符跳转到指针指向的值”一节中进一步讨论解引用运算符。
        *i += 50;
        println!("{}", *i)
    }

    // 我们曾经提到过动态数组只能存储相同类型的值。这个限制可能会带来不小的麻烦，实际工作中总是会碰到需要存储一些不同类型值的情况。
    // 幸运的是，当我们需要在动态数组中存储不同类型的元素时，可以定义并使用枚举来应对这种情况，因为枚举中的所有变体都被定义为了同一种枚举类型。
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
