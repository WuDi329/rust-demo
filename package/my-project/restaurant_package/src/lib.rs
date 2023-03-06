//在mod front_of_house后使用分号而不是代码块会让Rust前往与当前模块同名的文件中加载模块内容
mod front_of_house;

pub use crate::front_of_house::hosting;
pub use crate::test::test2;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    test2::testtest();
    test2::testtest();
    test2::testtest();
}