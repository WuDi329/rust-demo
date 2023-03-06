pub trait Messenger {

    // 它唯一的方法send可以接收self的不可变引用及一条文本消息作为参数
     fn send(&self, msg: &str);
   }
   
   pub struct LimitTracker<'a, T: 'a + Messenger> {
       messenger: &'a T,
       value: usize,
       max: usize,
   }
   
   impl<'a, T> LimitTracker<'a, T>
       where T: Messenger {
       pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
           LimitTracker {
               messenger,
               value: 0,
               max,
           }
       }
   
     pub fn set_value(&mut self, value: usize) {
           self.value = value;
   
           let percentage_of_max = self.value as f64 / self.max as f64;
   
           if percentage_of_max >= 1.0 {
               self.messenger.send("Error: You are over your quota!");
           } else if percentage_of_max >= 0.9 {
                self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
           } else if percentage_of_max >= 0.75 {
               self.messenger.send("Warning: You've used up over 75% of your quota!");
           }
       }
   }


   #[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

  struct MockMessenger {
    //   sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
      fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![])}
        }
    }

  impl Messenger for MockMessenger {
    // send方法接收的是不可变引用
        fn send(&self, message: &str) {
            // 随后的代码调用了RefCell<Vec<String>>类型的self.sent_messages的borrow_mut方法❸，
            // 来获取RefCell<Vec<String>>内部值（也就是动态数组）的可变引用。
            // 接着，我们便可以在动态数组的可变引用上调用push方法来存入数据，从而将已发送消息记录在案。

            // 这里同样维护了只能有一个可变引用的条件
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }
    }

    #[test]
  fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}