// 由于多个线程可以同时运行，所以将程序中的计算操作拆分至多个线程可以提高性能。
// 但这也增加了程序的复杂度，因为不同线程在执行过程中的具体顺序是无法确定的。

// 当多个线程以不一致的顺序访问数据或资源时产生的竞争状态（race condition）。
// 当两个线程同时尝试获取对方持有的资源时产生的死锁（deadlock），它会导致这两个线程无法继续运行。
// 只会出现在特定情形下且难以稳定重现和修复的bug。

// 1：1模型
// 许多操作系统都提供了用于创建新线程的API。
// 这种直接利用操作系统API来创建线程的模型常常被称作1:1模型，
// 它意味着一个操作系统线程对应一个语言线程。

// 绿色线程
// 也有许多编程语言提供了它们自身特有的线程实现，
// 这种由程序语言提供的线程常常被称为绿色线程（green thread），
// 使用绿色线程的语言会在拥有不同数量系统线程的环境下运行它们。
// 为此，绿色线程也被称为M:N模型，它表示M个绿色线程对应着N个系统线程，这里的M与N不必相等。

// Rust最重要的权衡因素：在于是否需要提供运行时支持。
// 运行时指语言中那些被包含在每一个可执行文件中的代码。不同的语言拥有不同大小的运行时代码.
// Rust会尽可能地保持几乎没有运行时的状态，这使我们可以方便地与C语言进行交互并获得较高的性能。



// 我们可以调用thread::spawn函数来创建线程，它接收一个闭包（在第13章中讨论过）作为参数，该闭包会包含我们想要在新线程（生成线程）中运行的代码。
use std::thread;

use std::time::Duration;

fn main() {
    // 需要注意的是，只要这段程序中的主线程运行结束，创建出的新线程就会相应停止，而不管它的打印任务是否完成。
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }


    // 使用join句柄等待所有线程结束
    // 我们可以通过将thread::spawn返回的结果保存在一个变量中，来避免新线程出现不执行或不能完整执行的情形。
    // thread::spawn的返回值类型是一个自持有所有权的JoinHandle，调用它的join方法可以阻塞当前线程直到对应的新线程运行结束。

    let result = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

        for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    // 如果将handle.join()放置到main函数的for循环之前会发生
    // 在这段代码中，由于主线程会等待新线程执行完毕后才开始执行自己的for循环，所以它的输出将不再出现交替的情形，如下所示：
    result.join().unwrap();



    // move 闭包
    // move闭包常常被用来与thread::spawn函数配合使用，它允许你在某个线程中使用来自另一个线程的数据。
    // 可以在闭包的参数列表前使用move关键字来强制闭包从外部环境中捕获值的所有权。

    // 不work的一种写法
    // closure may outlive the current function, but it borrows `v`, which is owned by the current function
    // Rust在推导出如何捕获v后决定让闭包借用v，因为闭包中的println! 只需要使用v的引用。
    // 但这就出现了一个问题：由于Rust不知道新线程会运行多久，所以它无法确定v的引用是否一直有效。
    let v = vec![1, 2, 3];

    // 通过在闭包前添加move关键字，我们会强制闭包获得它所需值的所有权，而不仅仅是基于Rust的推导来获得值的借用。
    let handle = thread::spawn(move|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    // 通过在闭包前添加move关键字，我们会强制闭包获得它所需值的所有权，而不仅仅是基于Rust的推导来获得值的借用。
}


