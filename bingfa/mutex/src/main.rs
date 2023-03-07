// 从某种程度上来说，任何编程语言中的通道都有些类似于单一所有权的概念，
// 因为你不应该在值传递给通道后再次使用它。
// 而基于共享内存的并发通信机制则更类似于多重所有权概念：多个线程可以同时访问相同的内存地址。
// 正如我们在第15章讨论的那样，我们可以通过智能指针实现多重所有权，但由于需要同时管理多个所有者，所以这会为系统增加额外的复杂性。

// 一个互斥体在任意时刻只允许一个线程访问数据。
// 为了访问互斥体中的数据，线程必须首先发出信号来获取互斥体的锁（lock）。
// 锁是互斥体的一部分，这种数据结构被用来记录当前谁拥有数据的唯一访问权。
// 通过锁机制，互斥体守护（guarding）了它所持有的数据。
use std::sync::Mutex;
use std::thread;
use std::rc::Rc;
use std::sync::Arc;

fn main() {

    // 与许多其他类型一样，我们可以使用关联函数new❶来创建Mutex<T>实例。
    
  let m = Mutex::new(5);

    {
        // 为了访问Mutex<T>实例中的数据，我们首先需要调用它的lock方法来获取锁❷。这个调用会阻塞当前线程直到我们取得锁为止。

        // 当前线程对于lock函数的调用会在其他某个持有锁的线程发生panic时失败。
        // 实际上，任何获取锁的请求都会在这种场景里以失败告终，所以示例中的代码选择使用unwrap在意外发生时触发当前线程的panic。
        // 一旦获取了锁，我们便可以将它的返回值num视作一个指向内部数据的可变引用❸。


        // 正如你可能会猜到的那样，Mutex<T>是一种智能指针。
        // 更准确地说，对lock的调用会返回一个名为MutexGuard的智能指针。
        // 这个智能指针通过实现Deref来指向存储在内部的数据，
        // 它还会通过实现Drop来完成自己离开作用域时的自动解锁操作。
      let mut num = m.lock().unwrap();
      *num = 6;
  }

  println!("m = {:?}", m);


  // 试着在多线程环境中使用Mutex<T>来共享数据。
  // 在接下来的例子中，我们会依次启动10个线程，并在每个线程中分别为共享的计数器的值加1。

// 为了让一个值能拥有多个所有权，使用了Rc进行修饰
//    let counter = Rc::new(Mutex::new(0));

//    // counter被移动到了handle指代的线程中。而这一移动行为阻止了我们在第二个线程中调用lock来再次捕获counter。 
//    let mut handles = vec![];
 

//  // 首先创建了一个名为counter的变量来存储持有i32值的Mutex<T>❶。
//  // 随后，我们通过迭代数字范围创建出了10个线程❷。
//      for _ in 0..10 {

//         let counter = Rc::clone(&counter);

//         // 在调用thread::spawn创建线程的过程中，我们给所有创建的线程传入了同样的闭包。
//        let handle = thread::spawn(move || {
//            let mut num = counter.lock().unwrap();
 
//            *num += 1;
//          });
//        handles.push(handle);
//      }
 
//      for handle in handles {
//        handle.join().unwrap();
//      }
 
//    println!("Result: {}", *counter.lock().unwrap());

   // 不幸的是，Rc<T>在跨线程使用时并不安全。
   // 当Rc<T>管理引用计数时，它会在每次调用clone的过程中增加引用计数，
   // 并在克隆出的实例被丢弃时减少引用计数，但它并没有使用任何并发原语来保证修改计数的过程不会被另一个线程所打断。
   // 这极有可能导致计数错误并产生诡异的bug，比如内存泄漏或值在使用时被莫名其妙地提前释放。
   // 我们需要的是一个行为与Rc<T>一致，且能够保证线程安全的引用计数类型。

   // 我们拥有一种被称为Arc<T>的类型，它既拥有类似于Rc<T>的行为，又保证了自己可以被安全地用于并发场景。
   // 它名称中的A代表着原子（atomic），表明自己是一个原子引用计数（atomically reference counted）类型。
   // 原子是一种新的并发原语，我们可以参考标准库文档中的std::sync::atomic部分来获得更多相关信息。
   // 你现在只需要知道：原子和原生类型的用法十分相似，并且可以安全地在多个线程间共享。

   let counter = Arc::new(Mutex::new(0));

   // counter被移动到了handle指代的线程中。而这一移动行为阻止了我们在第二个线程中调用lock来再次捕获counter。 
   let mut handles = vec![];
 

 // 首先创建了一个名为counter的变量来存储持有i32值的Mutex<T>❶。
 // 随后，我们通过迭代数字范围创建出了10个线程❷。
     for _ in 0..10 {

        let counter = Arc::clone(&counter);

        // 在调用thread::spawn创建线程的过程中，我们给所有创建的线程传入了同样的闭包。
       let handle = thread::spawn(move || {
           let mut num = counter.lock().unwrap();
 
           *num += 1;
         });
       handles.push(handle);
     }
 
     for handle in handles {
       handle.join().unwrap();
     }
 
   println!("Result: {}", *counter.lock().unwrap());


   // 你可能会注意到，虽然counter本身不可变，但我们仍然能够获取其内部值的可变引用。
   // 这意味着，Mutex<T>与Cell系列类型有着相似的功能，它同样提供了内部可变性。
   // 我们在第15章使用了RefCell<T>来改变Rc<T>中的内容，而本节按照同样的方式使用Mutex<T>来改变Arc<T>中的内容。

   // 使用Rc<T>会有产生循环引用的风险。两个Rc<T>值在互相指向对方时会造成内存泄漏。
   // 与之类似，使用Mutex<T>也会有产生死锁（deadlock）的风险。
   // 当某个操作需要同时锁住两个资源，而两个线程分别持有其中一个锁并相互请求另外一个锁时，这两个线程就会陷入无穷尽的等待过程。
   // 如果你对死锁感兴趣，不妨试着编写一个可能导致死锁的Rust程序。
   // 然后，你还可以借鉴其他语言中规避互斥体死锁的策略，并在Rust中实现它们。标准库API文档的Mutex<T>和MutexGuard页面为此提供了许多有用的信息。
   
}




