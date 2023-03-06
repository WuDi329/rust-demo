use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 因为生命周期也是泛型的一种，所以生命周期参数'a和泛型参数T都被放置到了函数名后的尖括号列表中。
fn longest_with_an_announcement_mine<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main(){

}