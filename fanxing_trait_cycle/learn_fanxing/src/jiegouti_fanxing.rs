// 在定义Point<T>时仅使用了一个泛型，这个定义表明Point<T>结构体对某个类型T是通用的。而无论具体的类型是什么，字段x与y都同时属于这个类型。
struct Point<T> {
    x: T,
    y: T,
}

//  我们必须紧跟着impl关键字声明T，以便能够在实现方法时指定类型Point<T>。
// 通过在impl之后将T声明为泛型，Rust能够识别出Point尖括号内的类型是泛型而不是具体类型。
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 打个比方，我们可以单独为Point<f32>实例而不是所有的Point<T>泛型实例来实现方法。
// 我们使用了这个具体的类型f32，这也意味着我们无须在impl之后声明任何类型了。
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointDouble<T, U> {
    x: T,
    y: U,
}

// 结构体定义中的泛型参数并不总是与我们在方法签名上使用的类型参数一致。
// 在某些情况下可能会有一部分泛型参数声明于impl关键字后，而另一部分则声明于方法定义中。
impl<T, U> PointDouble<T, U> {
    fn mixup<V, W>(self, other: PointDouble<V, W>) -> PointDouble<T, W> {
        PointDouble {
            x: self.x,
            y: other.y,
        }
    }
}




pub fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float: Point<f32> = Point { x: 1.0, y: 4.0 };
    let dis = both_float.distance_from_origin();
    println!("{}", dis);
    
    // 现在，所有的这些Point实例都是合法的了！你可以在定义中使用任意多个泛型参数，
    // 但要注意，过多的泛型会使代码难以阅读。
    // 通常来讲，当你需要在代码中使用很多泛型时，可能就意味着你的代码需要重构为更小的片段。
    let integer_and_float = PointDouble { x: 5, y: 4.0 };

    // 当你意识到自己的代码拥有多个结构体或枚举定义，且仅仅只有值类型不同时，你就可以通过使用泛型来避免重复代码。

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    //  let p1 = Point { x: 5, y: 10.4 };
    //  let p2 = Point { x: "Hello", y: 'c'};
   
    //  let p3 = p1.mixup(p2);
   
    //  println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// 当你使用泛型参数时，你也许会好奇这种机制是否存在一定的运行时消耗。
// 好消息是，Rust实现泛型的方式决定了使用泛型的代码与使用具体类型的代码相比不会有任何速度上的差异。
// 为了实现这一点，Rust会在编译时执行泛型代码的单态化（monomorphization）。
// 单态化是一个在编译期将泛型代码转换为特定代码的过程，它会将所有使用过的具体类型填入泛型参数从而得到有具体类型的代码。
// 它会寻找所有泛型代码被调用过的地方，并基于该泛型代码所使用的具体类型生成代码。

