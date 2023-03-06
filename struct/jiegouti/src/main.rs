
//我们使用了自持所有权的String类型而不是&str字符串切片类型。
//这是一个有意为之的选择，因为我们希望这个结构体的实例拥有自身全部数据的所有权。
//在这种情形下，只要结构体是有效的，那么它携带的全部数据也就是有效的。

//生命周期保证了结构体实例中引用数据的有效期不短于实例本身。你也许会尝试在没有生命周期的情形下，直接在结构体中声明引用字段：
// 
struct User <'a> {
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);



fn main(){
    //一旦实例可变，那么实例中的所有字段都将是可变的。Rust不允许我们单独声明某一部分字段的可变性
    let mut user = User {
        username: "wudi",
        email: "wuuudiii@qq.com",
        active: true,
        sign_in_count: 1
    };
    println!("{}", user.username);
    let email = String::from("diiiwuuu@gmail.com");
    // user.email = &email;
    println!("{}", user.email);
    let name = String::from("lalal");
    let email = String::from("wuu@tsinghua.edu.cn");
    let mut user1 = build_user(&email, &name);
    // let mut user2 = build_user_simple(email, name);

    // 定义元组结构体
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// 如果只有一个参数，那么User可以不加<'a>，甚至参数啥的都可以不包含'a
// 但是现在有两个
fn build_user<'a>(email: &'a str, username: &'a str) ->  User<'a> {
    User {
        email: email,
        username: "wd",
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_simple<'a>(email: &'a str, username: &'a str) -> User<'a> {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}