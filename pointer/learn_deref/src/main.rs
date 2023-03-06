struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
      MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {

    // 下面定义了Deref trait的一个关联类型。
    // 关联类型是一种稍微有些不同的泛型参数定义方式。
    // 我们会在第19章对这一特性进行深入的讨论，现在先忽略它就好。
  type Target = T;

    fn deref(&self) -> &T {
        // 这意味着deref会返回一个指向值的引用，进而允许调用者通过*运算符访问值
      &self.0
    }
}

fn hello(str: &str){
    println!("{}", str);
}

fn main() {
    // learn_deref 的设计目的在于对智能指针实现和普通指针一样的功能

    // Rust使用*运算符来替代deref方法和另外一个朴素的解引用操作，这样我们就不用考虑是否需要调用deref方法了。
    // 这一特性使我们可以用完全相同的方式编写代码来处理常规引用及实现了Deref trait的类型。
  let x = 5;
  let y = &x;

  assert_eq!(5, x);
  // can't compare `{integer}` with `&{integer}`
  assert_eq!(5, *y);

  let z = Box::new(x);

  assert_eq!(5, *z);

  // can't compare `{integer}` with `Box<{integer}>`

  let a = MyBox::new(x);

  assert_eq!(5, *a);

  // 解引用转换（deref coercion）是Rust为函数和方法的参数提供的一种便捷特性。
  // 当某个类型T实现了Deref trait时，它能够将T的引用转换为T经过Deref操作后生成的引用。
  // 当我们将某个特定类型的值引用作为参数传递给函数或方法，但传入的类型与参数类型不一致时，解引用转换就会自动发生。
  // 编译器会插入一系列的deref方法调用来将我们提供的类型转换为参数所需的类型。

  // 因为我们在示例15-10中为MyBox<T>实现了Deref trait，所以Rust可以通过调用deref来将&MyBox<String>转换为&String。
  // 因为标准库为String提供的Deref实现会返回字符串切片（你可以在Deref的API文档中看到这一信息），
  // 所以Rust可以继续调用deref来将&String转换为&str，并最终与hello函数的定义相匹配。

  // deref能够将T的引用转换为T经过Deref操作后生成的引用。
  let str = MyBox::new(String::from("lallala"));
  hello(&str);


}