// recursive type `List` has infinite size
// insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable: `Box<`, `>`
// 建议中的indirection（间接）意味着，我们应该改变数据结构来存储指向这个值的指针，而不是直接地存储这个值。

// 因为Box<T>是一个指针，所以Rust总是可以确定一个Box<T>的具体大小。
// 指针的大小总是恒定的，它不会因为指向数据的大小而产生变化。
// 这也意味着我们可以在Cons变体中存放一个Box<T>而不是直接存放另外一个List值。

#[derive(Debug)]
enum List<T> {
    // 装箱正好能够被用在类似于链接列表这类仅仅需要间接访问的场景中
    Cons(T, Box<List<T>>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // 新的Cons变体需要一部分存储i32的空间和一部分存储装箱指针数据的空间。
    // 另外，由于Nil变体没有存储任何值，所以它需要的空间比Cons变体小。
    // 现在，我们知道任意的List值都只需要占用一个i32值加上一个装箱指针的大小。
    // 通过使用装箱，我们打破了无限递归的过程，进而使编译器可以计算出存储一个List值需要多大的空间。
    let list = Cons(1, 
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
    println!("{:?}", list);
}
