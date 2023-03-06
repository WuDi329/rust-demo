pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello")
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}





#[cfg(test)]
mod tests {
    // 注意，我们在tests模块中新增加了一行：use super::*;❶。
    // tests模块与其他模块没有任何区别，它同样遵循第7章的“用于在模块树中指明条目的路径”一节中介绍的可见性规则。
    // 因为tests是一个内部模块，所以我们必须将外部模块中的代码导入内部模块的作用域中。
    // 这里使用了通配符（*）让外层模块所定义的全部内容在tests模块中都可用。
    use super::*;


    // 这一行出现了#[test]标注：它将当前的函数标记为一个测试，并使该函数可以在测试运行过程中被识别出来。

    // test result: ok.❸表示该集合中的所有测试均成功通过，1 passed; 0 failed则统计了通过和失败的测试总数。
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn not_work() {
        let result = add(3, 4);
        assert_eq!(result, 5);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
           let smaller = Rectangle { length: 5, width: 8 };
   
           // 使用assert!判断括号里的真假
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn not_equal() {
        let a = 3;
        // 从本质上来看，assert_eq! 和assert_ne! 宏分别使用了==和!=运算符来进行判断，
        // 并在断言失败时使用调试输出格式（{:?}）将参数值打印出来。
        // 这意味着它们的参数必须同时实现PartialEq和Debug这两个trait。

        // 由于这两个trait都是可派生trait，
        // 所以它们一般可以通过在自定义的结构体或枚举的定义的上方添加#[derive(PartialEq, Debug)]标注来自动实现这两个trait。
        assert_ne!(a,2);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(50);
    }


    // 不要在使用Result<T, E>编写的测试上标注#[should_panic]。在测试运行失败时，我们应当直接返回一个Err值。
    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

