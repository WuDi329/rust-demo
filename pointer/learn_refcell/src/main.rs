// 内部可变性（interior mutability）是Rust的设计模式之一，
// 它允许你在只持有不可变引用的前提下对数据进行修改；通常而言，类似的行为会被借用规则所禁止。
// 为了能够改变数据，内部可变性模式在它的数据结构中使用了unsafe（不安全）代码来绕过Rust正常的可变性和借用规则。
// 我们会在第19章学习如何使用不安全代码。
// 假如我们能够保证自己的代码在运行时符合借用规则，
// 那么就可以在即使编译器无法在编译阶段保证符合借用规则的前提下，
// 也能使用那些采取了内部可变性模式的类型。
// 实现过程中涉及的那些不安全代码会被妥善地封装在安全的API内，而类型本身从外部看来依然是不可变的。

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;


fn main() {


    // 对于使用一般引用和Box<T>的代码，Rust会在编译阶段强制代码遵守这些借用规则。
    // 而对于使用RefCell<T>的代码，Rust则只会在运行时检查这些规则，
    // 并在出现违反借用规则的情况下触发panic来提前中止程序。


    // 与Rc<T>相似，RefCell<T>只能被用于单线程场景中。强行将它用于多线程环境中会产生编译时错误。

    // 在某些特定情况下，我们也会需要一个值在对外保持不可变性的同时能够在方法内部修改自身。
    // 除了这个值本身的方法，其余的代码则依然不能修改这个值。
    // 使用RefCell<T>就是获得这种内部可变性的一种方法。
    println!("Hello, world!");

     let value = Rc::new(RefCell::new(5));

     let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
   
       let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
       let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
   
   // 创建完a、b、c这3个列表后，我们通过调用borrow_mut来将value指向的值增加10❸。
   // 注意，这里使用了自动解引用功能（在第5章讨论过）来将Rc<T>解引用为RefCell<T>。
   // borrow_mut方法会返回一个RefMut<T>智能指针，我们可以使用解引用运算符来修改其内部值。
     *value.borrow_mut() += 10;
   
       println!("a after = {:?}", a);
       println!("b after = {:?}", b);
       println!("c after = {:?}", c);

       // 标准库还提供了其他一些类型来实现内部可变性，例如与RefCell<T>十分类似的Cell<T>，但相比于前者通过借用来实现内部数据的读写，Cell<T>选择了通过复制来访问数据。另外还有在第16章会讨论到的Mutex<T>，它被用于实现跨线程情形下的内部可变性模式。请参考标准库文档来了解有关这些类型有哪些区别的更多信息。
}
