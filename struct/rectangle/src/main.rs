#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

//由于方法的声明过程被放置在impl Rectangle块中，所以Rust能够将self的类型推导为Rectangle。
impl Rectangle {
    // 方法可以在声明时选择获取self的所有权，
    // 也可以像本例一样采用不可变的借用&self，
    // 或者采用可变的借用&mut self。
    // 总之，就像是其他任何普通的参数一样。
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool{
        if self.width >= rectangle.width && self.height >= rectangle.height {
            return true;
        }
        return false;
    }

    // 除了方法，impl块还允许我们定义不用接收self作为参数的函数。
    // 由于这类函数与结构体相互关联，所以它们也被称为关联函数（associated function）。
    // 我们将其命名为函数而不是方法，是因为它们不会作用于某个具体的结构体实例。
    // 你曾经接触过的String::from就是关联函数的一种。
    fn square(size: u32) -> Rectangle{
        return Rectangle{ width: size, height: size};
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let mut rectangle = Rectangle{
        width: 50,
        height: 30
    };

    let rectangle2 = Rectangle{
        width:10,
        height:20
    };

    //我们可以在类型名称后添加::来调用关联函数，就像let sq = Rectangle::square(3);一样。这个函数位于结构体的命名空间
    let square = Rectangle::square(3);

    println!("rectangle can hold square? {}", rectangle.can_hold(&square));

    println!(
        "The area of the rectangle is {} square pixels.",
        square.area()
    );

    // 设计了一种名为自动引用和解引用的功能。
    // 方法调用是Rust中少数几个拥有这种行为的地方之一。
    // 它的工作模式如下：当你使用object.something()调用方法时，
    // Rust会自动为调用者object添加&、&mut或*，以使其能够符合方法的签名。
    // 换句话说，下面两种方法调用是等价的：
    // p1.distance(&p2);
    // (&p1).distance(&p2);
    // 这种自动引用行为之所以能够行得通，是因为方法有一个明确的作用对象：self的类型。
    // 在给出调用者和方法名的前提下，Rust可以准确地推导出方法是否是只读的（&self），
    // 是否需要修改数据（&mut self），是否会获取数据的所有权（self）。
    // 这种针对方法调用者的隐式借用在实践中可以让所有权系统更加友好且易于使用。
    
    println!("rectangle can hold rectangle2? {}", rectangle.can_hold(&rectangle2));
        
    //当你使用object.something()调用方法时，Rust会自动为调用者object添加&、&mut或*，以使其能够符合方法的签名。
    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );

    // 这里是直接输出结构体的内容
    // 如果直接println，由于结构体的特性，需要自己实现display这个trait。
    // 但是同样有便捷的解决方式：
    // 我们把标识符号:?放入了花括号中，它会告知println! 当前的结构体需要使用名为Debug的格式化输出。
    // 为了完成该声明，我们在结构体定义前添加了#[derive(Debug)]注解
    println!("{:#?}", rectangle);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}



fn area_tuple(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}