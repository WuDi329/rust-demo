fn main() { 
    //我们需要将变量s声明为mut，即可变的。
    let mut s1 = String::from("hello"); 
 
    let len = calculate_length(&s1); 
 
    println!("The length of '{}' is {}.", s1, len); 

    // 其次，我们使用&mut s来给函数传入一个可变引用，
    //可变引用在使用上有一个很大的限制：
    //对于特定作用域中的特定数据来说，一次只能声明一个可变引用。以下代码尝试违背这一限制，则会导致编译错误：

    let a = &mut s1;
    //cannot borrow `s1` as mutable more than once at a time
    //cannot borrow `s1` as immutable because it is also borrowed as mutable \nimmutable borrow occurs here
    //不能在拥有不可变引用的同时创建可变引用。不可变引用的用户可不会希望他们眼皮底下的值突然发生变化！
    // let b = &s1;
    // change(b);
    change(a);

    //数据竞争出现的场景：同时满足以下几点
    //两个或两个以上的指针同时访问同一空间。
    //其中至少有一个指针会向空间中写入数据。
    //没有同步数据访问的机制。

    // 在任何一段给定的时间里，你要么只能拥有一个可变引用，要么只能拥有任意数量的不可变引用。
    // 引用总是有效的


    // 可以通过花括号来创建一个新的作用域范围。这就使我们可以创建多个可变引用，当然，这些可变引用不会同时存在：
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
            
    } // 由于 r1 在这里离开了作用域，所以我们可以合法地再创建一个可变引用。

    let r2 = &mut s;


    //悬垂引用
    let reference_to_notiong = dangle();
} 
 
//由于引用不持有值的所有权，所以当引用离开当前作用域时，它指向的值也不会被丢弃。
fn calculate_length(s: &String) -> usize { 
    s.len() 
} 

//cannot borrow `*s` as mutable, as it is behind a `&` reference
//并将函数签名修改为some_string: &mut String来使其可以接收一个可变引用作为参数。
fn change(s: &mut String) {
    s.push_str(",world");
}

//dangle会返回一个指向String的引用
pub fn dangle() -> String {
    let s = String::from("hello");
    s //把s的引用返回给调用者
} //变量s离开作用域并且随之销毁，指向的内存也就不再有效了