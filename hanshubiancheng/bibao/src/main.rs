// Rust中的闭包是一种可以存入变量或作为参数传递给其他函数的匿名函数。
// 你可以在一个地方创建闭包，然后在不同的上下文环境中调用该闭包来完成运算。
// 和一般的函数不同，闭包可以从定义它的作用域中捕获值。
// 我们将展示如何运用闭包的这些特性来实现代码复用和行为自定义。

// 闭包是匿名函数，主要解决的问题：代码复用&行为自定义
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    // let example_closure = |x| x;

    // let s = example_closure(String::from("hello"));
    // // 如果调用闭包两次，第一次使用String类型的参数，而第二次使用u32类型的参数，那么就会发生编译错误。
    // let n = example_closure(5);

    let x = String::from("wudi");

    // 在上面的代码中，即使x不是equal_to_x的参数，equal_to_x闭包也可以使用定义在同一个作用域中的变量x。
    let equal_to_x = |z: &str| {x + z};


    let name = String::from("ud");
    let res = equal_to_x(&name);

    println!("{}", res);

    // value borrowed after move，在使用闭包的情况下也会产生所有权的转移
    // 如果传递的是引用，那就不会
    println!("{}", name);

    let y = 4;

    // assert!(equal_to_x(y));

    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));

}


use std::thread;
use std::time::Duration;

// 模拟一个非常复杂的函数，可能开销好几秒来做一件事情
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}


// 所谓的核心的计算方法
// 当前的计算很麻烦
// 为了在遇到这种情况时简化更新流程，我们想通过重构代码来使它只调用simulated_expensive_calculation函数一次。
// 另外，我们还希望在不引入其他函数调用的前提下，优化那处调用了两次复杂算法函数的代码块。
// 换句话说，我们希望在不必要时避免调用这个耗时的计算函数，在必要时也最多只调用一次。
fn generate_workout(intensity: u32, randon_number: i32) {
    if intensity < 25 {
        println!("{}", simulated_expensive_calculation(intensity));
        println!("{}", simulated_expensive_calculation(intensity));
    } else {
        if randon_number == 3 {
            println!("hahahah");
        } else {
            println!("{}", simulated_expensive_calculation(intensity));
        }
    }
}

// 第一种更新方法：抽象出变量
// 但不幸的是，我们在所有调用了这个函数的情况下都要等待结果，这对于无须结果值的内层if代码块来讲显得异常浪费。
fn generate_workout_update(intensity: u32, randon_number: i32) {
    let number = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!("{}", number);
        println!("{}", number);
    } else {
        if randon_number == 3 {
            println!("hahahah");
        } else {
            println!("{}", number);
        }
    }
}

// 第二种更新方法：我们希望在程序中将代码定义在一处，但只在真正需要结果时才执行相关代码。而这正是闭包的用武之地！
// 相较于每次在if块之前调用simulated_expensive_calculation函数，
// 我们可以定义一个闭包，并将闭包而不是函数的计算结果存储在变量中，如下。
// 实际上，我们可以直接将simulated_expensive_ calculation的整个函数体都移动到闭包中。
fn generate_workout_update2(intensity: u32, randon_number: i32) {

    // 闭包的定义放置在=之后，它会被赋值给语句左侧的expensive_closure变量。
    // 为了定义闭包，我们需要以一对竖线（|）开始，并在竖线之间填写闭包的参数

    // 注意，这条let语句意味着expensive_closure变量存储了一个匿名函数的定义，而不是调用该匿名函数而产生的返回值。
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // 但是，这样的改动让示例13-3中的老问题重新出现了：
    // 我们依然在第一个if块中调用了两次闭包，也就是执行了两次耗时的计算操作，
    // 进而导致用户的等待时间不合理地被延长了两倍。
    // 当然，在if块中定义一个局部变量来存储闭包结果可以解决这个问题，
    // 但你也可以利用闭包的特性提供另一种解决方案。
    // 现在，先来看一看为什么闭包定义及其相关trait中都没有出现任何的类型标注。
    if intensity < 25 {
        println!("{}", expensive_closure(intensity));
        println!("{}", expensive_closure(intensity));
    } else {
        if randon_number == 3 {
            println!("hahahah");
        } else {
            println!("{}", expensive_closure(intensity));
        }
    }
}

fn generate_workout_update3(intensity: u32, randon_number: i32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("{}", expensive_result.value(intensity));
        println!("{}", expensive_result.value(intensity));
    } else {
        if randon_number == 3 {
            println!("hahahah");
        } else {
            println!("{}", expensive_result.value(intensity));
        }
    }
}

// 幸运的是，我们还有另一种可用的解决方案：
// 创建一个同时存放闭包及闭包返回值的结构体。
// 这个结构体只会在我们需要获得结果值时运行闭包，并将首次运行闭包时的结果缓存起来，这样余下的代码就不必再负责存储结果，而可以直接复用该结果。
// 这种模式一般被称作 记忆化（memoization）或 惰性求值（lazy evaluation）。

// 。但需要注意的是，每一个闭包实例都有它自己的匿名类型。
// 换句话说，即便两个闭包拥有完全相同的签名，它们的类型也被认为是不一样的。
// 为了在结构体、枚举或函数参数中使用闭包，我们需要使用在第10章讨论过的泛型及trait约束。

// 我们会在Fn的trait约束中添加代表了闭包参数和闭包返回值的类型。
// 在这个例子中，闭包有一个u32类型的参数并返回一个u32值，因此我们指定的trait约束就是Fn(u32) -> u32。
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,

    // 另外一个字段value的类型是Option<u32>。
    // 在运行闭包之前，value会被初始化为None。
    // 而当使用Cacher的代码请求闭包的执行结果时，
    // Cacher会运行闭包并将结果存储在value的Some变体中。
    // 之后，如果代码重复请求闭包的结果，
    // Cacher就可以避免再次运行闭包，
    // 而将缓存在Some变体中的结果返回给调用者。
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
 fn new(calculation: T) -> Cacher<T> {
     Cacher {
            calculation,
            value: None,
        }
    }

    // 这里的问题在于我们第一次使用1作为参数来执行c.value时，Cacher实例就将Some(1)存储在了self.value中。
    // 在这之后，无论我们在调用value方法时传入的值是什么，它都只会返回1。


    // 解决这个问题的方法是让Cacher存储一个哈希表而不是单一的值。
    // 这个哈希表使用传入的arg值作为关键字，
    // 并将关键字调用闭包后的结果作为对应的值。
    // 相应地，value方法不再简单地判断self.value的值是Some还是None，
    // 而是会检查哈希映射里是否存在arg这个关键字。
    // 如果存在的话，Cacher就直接返回对应的值；
    // 如果不存在的话，则调用闭包，使用arg关键字将结果存入哈希表之后再返回。
 fn value(&mut self, arg: u32) -> u32 {
        match self.value {
         Some(v) => v,
         None => {
            let v = (self.calculation)(arg);
            self.value = Some(v);
            v
        },
        }
    }
}

// 但除此之外，闭包还有一项函数所不具备的功能：它们可以捕获自己所在的环境并访问自己被定义时的作用域中的变量。
// 闭包可以通过3种方式从它们的环境中捕获值，这和函数接收参数的3种方式是完全一致的：
// 获取所有权、可变借用及不可变借用。这3种方式被分别编码在如下所示的3种Fn系列的 trait中：
// FnOnce意味着闭包可以从它的封闭作用域中，也就是闭包所处的环境中，消耗捕获的变量。为了实现这一功能，闭包必须在定义时取得这些变量的所有权并将它们移动至闭包中。这也是名称FnOnce中Once一词的含义：
// • 因为闭包不能多次获取并消耗掉同一变量的所有权，所以它只能被调用一次。
// • FnMut可以从环境中可变地借用值并对它们进行修改。
// • Fn可以从环境中不可变地借用值。

// 假如你希望强制闭包获取环境中值的所有权，
// 那么你可以在参数列表前添加move关键字。
// 这个特性在把闭包传入新线程时相当有用，
// 它可以将捕获的变量一并移动到新线程中去。

fn test_bibao() {

    // 在上面的代码中，即使x不是equal_to_x的参数，equal_to_x闭包也可以使用定义在同一个作用域中的变量x。
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}