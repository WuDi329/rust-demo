mod jiegouti_fanxing;
fn main() {
    // 高效地处理重复概念，并为此提供各种各样的工具
    // 在Rust中，泛型（generics）就是这样一种工具。泛型是具体类型或其他属性的抽象替代
    // 在编写代码时，我们可以直接描述泛型的行为，或者它与其他泛型产生的联系，而无须知晓它在编译和运行代码时采用的具体类型。
    println!("Hello, world!");

    let number_list = vec![34, 50, 25, 100, 65];
   
    let mut largest = number_list[0];
   
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
   
    println!("The largest number is {}", largest);


    // 使用函数进行优化
    let number_list = vec![34, 50, 25, 100, 65];

    let result = get_largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = get_largest(&number_list);
    println!("The largest number is {}", result);

    // 使用泛型进行优化
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);


    // 使用泛型进行优化
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_fanxing(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_fanxing(&char_list);
    println!("The largest char is {}", result);

    jiegouti_fanxing::main();
}

fn get_largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 当使用泛型来定义一个函数时，我们需要将泛型放置在函数签名中通常用于指定参数和返回值类型的地方。
// 以这种方式编写的代码更加灵活，并可以在不引入重复代码的同时向函数调用者提供更多的功能。
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}



// 当我们需要在函数体中使用参数时，我们必须要在签名中声明对应的参数名称，以便编译器知晓这个名称的含义。
// 类似地，当我们需要在函数签名中使用类型参数时，也需要在使用前声明这个类型参数的名称。
// 为了定义泛型版本的largest函数，类型名称的声明必须被放置在函数名与参数列表之间的一对尖括号<>中

// 简单来讲，这个错误表明largest函数中的代码不能适用于T所有可能的类型。
// 因为函数体中的相关语句需要比较类型T的值，这个操作只能被用于可排序的值类型。



// 假如我们不希望使largest函数只能使用那些实现了Copy trait的类型，
// 那么可以用Clone来替换T trait约束中的Copy。
// 接着，当需要在largest函数中取得切片中某个值的所有权时，
// 我们就可以使用克隆方法。当然，一旦搜索对象是类似于String之类的存储在堆上的类型时，
// 使用clone函数就意味着我们会执行更多堆分配操作，而当需要处理大量数据时，执行堆分配可能会相当缓慢。
fn largest_fanxing<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 另一种可能的largest实现方式是返回切片中T值的引用。
// 假如将返回类型从T修改为&T，
// 并修改函数体使其返回一个引用，
// 那么我们就不再需要Clone或Copy来进行trait约束了，同时可以避免执行堆分配操作。不妨自己尝试着实现一下这种方案吧！

fn largest_fanxing_yinyong<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}