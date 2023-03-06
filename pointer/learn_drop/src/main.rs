struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
      println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {

    // 它允许我们在变量离开作用域时执行某些自定义操作。
    // 你可以为任意类型实现一个Drop trait，它常常被用来释放诸如文件、网络连接等资源。
    // 我们之所以选择在智能指针的上下文中介绍Drop，是因为几乎每一种智能指针的实现都会用到这一trait。
    // 例如，Box<T>通过自定义Drop来释放装箱指针指向的堆内存空间。
    println!("Hello, world!");

    // Drop trait要求实现一个接收self可变引用作为参数的drop函数。

     let c = CustomSmartPointer { data: String::from("my stuff") };
     let d = CustomSmartPointer { data: String::from("other stuff") };
     println!("CustomSmartPointers created.");

     // 我们倒是常常会碰到需要提前清理一个值的情形。
     // 其中一个例子就是使用智能指针来管理锁时：你也许会希望强制运行drop方法来提前释放锁，
     // 从而允许同一作用域内的其他代码来获取它。
     // Rust并不允许我们手动调用Drop trait的drop方法；
     // 但是，你可以调用标准库中的std::mem::drop函数来提前清理某个值。

     // 因为Rust已经在main函数结尾的地方自动调用了drop，所以它不允许我们再次显式地调用drop。
     // 这种行为会试图对同一个值清理两次而导致重复释放（double free）错误。
     drop(c);
     println!("在结束main之前提前销毁c");
}
