// 消息传递（message passing）机制来保证并发安全正在变得越来越流行。
// 在这种机制中，线程或actor之间通过给彼此发送包含数据的消息来进行通信。

// Rust在标准库中实现了一个名为通道（channel）的编程概念，它可以被用来实现基于消息传递的并发机制。
// 你可以将它想象为有活水流动的通道，比如小溪或河流。只要你将橡皮鸭或小船这样的东西放入其中，它就会顺流而下抵达水路的终点。

// 编程中的通道由发送者（transmitter）和接收者（receiver）两个部分组成。
// 发送者位于通道的上游，也就是你放置橡皮鸭的地方；而接收者则位于通道的下游，也就是橡皮鸭到达的地方。
// 某一处代码可以通过调用发送者的方法来传送数据，而另一处代码则可以通过检查接收者来获取数据。
// 当你丢弃了发送者或接收者的任何一端时，我们就称相应的通道被关闭（closed）了。

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 上面的代码使用mpsc::channel函数创建了一个新的通道。
    // 路径中的mpsc是英文“multiple producer, single consumer”（多个生产者，单个消费者）的缩写。
    // 函数mpsc::channel会返回一个含有发送端与接收端的元组。代码中用来绑定它们的变量名称为tx和rx，这也是在许多场景下发送者与接收者的惯用简写。
    let (tx, rx) = mpsc::channel();

//     // 接下来，让我们将发送端移动到新线程中，并接着发送一个字符串来完成新线程与主线程的通信
    thread::spawn(move ||{
        let val = String::from("hi");
        // 我们使用move关键字将tx移动到了闭包的环境中。新线程必须拥有通道发送端的所有权才能通过通道来发送消息。
        // send函数会获取参数的所有权，并在参数传递时将所有权转移给接收者。
        // 这可以阻止我们意外地使用已经发送的值，所有权系统会在编译时确保程序的每个部分都是符合规则的。
        tx.send(val).unwrap();
    });


    // 通道的接收端有两个可用于获取消息的方法：recv和try_recv。
    // 我们使用的recv（也就是receive的缩写）会阻塞主线程的执行直到有值被传入通道。
    // 一旦有值被传入通道，recv就会将它包裹在Result<T, E>中返回。
    // 而如果通道的发送端全部关闭了，recv则会返回一个错误来表明当前通道再也没有可接收的值。

    // try_recv方法不会阻塞线程，它会立即返回Result<T, E>：当通道中存在消息时，返回包含该消息的Ok变体；否则便返回Err变体。
    // 当某个线程需要一边等待消息一边完成其他工作时，try_recv方法会非常有用。
    // 我们可以编写出一个不断调用try_recv方法的循环，并在有消息到来时对其进行处理，而在没有消息时执行其他指令。
    let recived = rx.recv().unwrap();
    println!("got {}", recived);

    // 新线程现在会发送多条信息，并在每次发送后暂停1秒钟。
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

// 在主线程中，我们会将rx视作迭代器，而不再显式地调用recv函数。迭代中的代码会打印出每个接收到的值，并在通道关闭时退出循环。
    for received in rx {
        println!("Got: {}", received);
    }


    let (tx, rx) = mpsc::channel();

let tx1 = mpsc::Sender::clone(&tx);
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {}", received);
}

// --略--
}

}



