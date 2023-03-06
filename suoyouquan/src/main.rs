

pub mod function;
mod yinyong;
mod qiepian;

fn main() {
    let mut s = String::from("hello");
    s.push_str(",world!");
    println!("{}", s);

    //需要注意的是，我们没有复制指针指向的堆数据。
    //使用了新的术语移动（move）来描述这一行为
    //另外一个设计原则：Rust永远不会自动地创建数据的深度拷贝
    let s2 = s.clone();
    println!("s = {}, s2 = {}", s, s2);

    let x = 3;
    let y = x;
    //因为类似于整型的类型可以在编译时确定自己的大小，并且能够将自己的数据完整地存储在栈中，对于这些值的复制操作永远都是非常快速的。
    //Copy的trait，它可以用于整数这类完全存储在栈上的数据类型


    // Rust提供了一个名为Copy的trait，它可以用于整数这类完全存储在栈上的数据类型（我们会在第10章详细地介绍trait）。
    // 一旦某种类型拥有了Copy这种trait，那么它的变量就可以在赋值给其他变量之后保持可用性。
    //任何简单标量的组合类型都可以是Copy的，任何需要分配内存或某种资源的类型都不会是Copy的。
    println!("x = {}, y = {}", x, y);

    //值传递给函数在语义上类似于对变量进行赋值。将变量传递给函数将会触发移动或复制，就像是赋值语句一样。
    takes_ownership(s2);

    // println!("{}", s2);

    makes_copy(x);

    // let str = yinyong::dangle();
    // println!("{}", str);

    qiepian::main()

    

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s //把s的引用返回给调用者
// } //变量s离开作用域并且随之销毁，指向的内存也就不再有效了

// fn xuanchui() -> & 'static str {
//     let str = String::from("lallala");
//     &str
// }
