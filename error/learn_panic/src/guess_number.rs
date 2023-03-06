pub struct Guess {
    value: i32,
}

// 我们可以创建一个新的类型，并在创建的类型实例的函数中对值进行有效性检查。
// 这样就可以在函数签名中安全地使用新类型，而无须担心我们所接收的值的有效性了。
impl Guess {
  pub fn new(value: i32) -> Guess {
    // 假如value没有通过测试，我们就触发panic! 。
    // 这会警告调用这段代码的程序员出现了一个需要修复的bug，
    // 因为使用超出范围的value来创建Guess违反了Guess::new所依赖的约定。
      if value < 1 || value > 100 {
          panic!("Guess value must be between 1 and 100, got {}.", value);
        }

      Guess {
            value
        }
    }


    //最后，我们实现了一个value方法，它仅有一个参数用于借用self，并返回一个i32类型的值。
    // 这类方法有时也被称作读取接口（getter），因为它的功能就在于读取相应字段内的数据并返回。
    // 因为Guess中的value字段是私有的，所以我们有必要提供这类公共方法用于访问数据。
    // 而之所以将value字段设置为私有的，是因为我们不允许使用Guess结构体的代码随意修改value中的值：
    // 模块外的代码必须使用Guess::new函数来创建新的Guess实例，这就确保了所有Guess实例中的value都可以在Guess::new函数中进行有效性检查。
  pub fn value(&self) -> i32 {
        self.value
    }
}