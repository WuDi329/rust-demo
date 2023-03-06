// 在某些场景中，单个值也可能同时被多个所有者持有。
// 例如，在图数据结构中，多个边可能会指向相同的节点，
// 而这个节点从概念上来讲就同时属于所有指向它的边。
// 一个节点只要在任意指向它的边还存在时就不应该被清理掉。 

// Rust提供了一个名为Rc<T>的类型来支持多重所有权，
// Rc<T>类型的实例会在内部维护一个用于记录值引用次数的计数器，从而确认这个值是否仍在使用。
// 如果对一个值的引用次数为零
// 那么就意味着这个值可以被安全地清理掉，而不会触发引用失效的问题。


enum List<T>{
    link(T, Box<List<T>>),
    Nil
}

enum List_rc<T> {
    link(T, Rc<List_rc<T>>),
    Nil
}

use std::rc::Rc;

use crate::List::link;
use crate::List::Nil;

use crate::List_rc::{link as link_rc, Nil as Nil_rc};

fn main() {
    // 当你希望将堆上的一些数据分享给程序的多个部分同时使用，
    // 而又无法在编译期确定哪个部分会最后释放这些数据时，我们就可以使用Rc<T>类型。
    // 相反地，如果我们能够在编译期确定哪一部分会最后释放数据，
    // 那么就只需要让这部分代码成为数据的所有者即可，
    // 仅仅靠编译期的所有权规则也可以保证程序的正确性。
    let public = link(2, 
        Box::new(link(3, 
            Box::new(Nil))));

    // Cons变体持有它存储的数据。因此，整个public列表会在我们创建b列表时❶被移动至a中。
    // 换句话说，a 列表持有了puiblic列表的所有权。
    // 当我们随后再次尝试使用public来创建b列表时❷就会出现编译错误，因为public已经被移走了。
    let a = link(0, Box::new(public));
    // let b = link(1, Box::new(public));


    // 我们可以将List中的Box<T>修改为Rc<T>，如示例15-18所示。在这段新的代码中，每个Cons变体都会持有一个值及一个指向List的Rc<T>。我们只需要在创建b的过程中克隆a的Rc<List>智能指针即可，而不再需要获取a的所有权。
    // 这会使a和b可以共享Rc<List>数据的所有权，并使智能指针中的引用计数从1增加到2。
    let public2 = Rc::new(link_rc(2,
        Rc::new(link_rc(3,
            Rc::new(Nil_rc)))));

// clone最外面要放的是Rc<>

// 你可以在这里调用a.clone()而不是Rc::clone(&a)来实现同样的效果，但Rust的惯例是在此场景下使用Rc::clone，
// 因为Rc::clone不会执行数据的深度拷贝操作，这与绝大多数类型实现的clone方法明显不同
// 。调用Rc::clone只会增加引用计数，而这不会花费太多时间。
// 但与此相对的是，深度拷贝则常常需要花费大量时间来搬运数据。
// 因此，在引用计数上调用Rc::clone可以让开发者一眼就区分开“深度拷贝”与“增加引用计数”这两种完全不同的克隆行为。
// 当你需要定位存在性能问题的代码时，就可以忽略Rc::clone而只需要审查剩余的深度拷贝克隆行为即可。
    let b = link_rc(3, Rc::clone(&public2));
    let c = link_rc(4, Rc::clone(&public2));

    let a = Rc::new(a);
    // let a = link_rc(0, a);



    // Rc<T>通过不可变引用使你可以在程序的不同部分之间共享只读数据。
    // 如果Rc<T>也允许你持有多个可变引用的话，
    // 那么它就会违反在第4章讨论过的其中一个借用规则：
    // 多个指向同一区域的可变借用会导致数据竞争及数据不一致。
    // 但在实际开发中，允许数据可变无疑是非常有用的！
    // 因此，我们接下来将要讨论内部可变性模式及RefCell<T>类型，
    // 该类型可以与Rc<T>联合使用来绕开不可变的限制。
    let a = Rc::new(link_rc(5, Rc::new(link_rc(10, Rc::new(Nil_rc)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = link_rc(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = link_rc(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

}
