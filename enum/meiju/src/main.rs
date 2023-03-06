// 目前有两种被广泛使用的IP地址标准：IPv4和IPv6
// 只需要处理这两种情形，所以可以将所有可能的值枚举出来，这也正是枚举名字的由来。

//一个IP地址要么是IPv4的，要么是IPv6的，没有办法同时满足两种标准。
//枚举的值也只能是变体中的一个成员
// 所以当我们需要在代码中处理IP地址时，应该将它们视作同一种类型

// 我们可以通过定义枚举IpAddrKind来表达这样的概念，
// 声明该枚举需要列举出所有可能的IP地址种类—V4和V6，
// 这也就是所谓的枚举变体（variant）：
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// 实际上，枚举允许我们直接将其关联的数据嵌入枚举变体内。
// 我们可以使用枚举来更简捷地表达出上述概念，而不用将枚举集成至结构体中。
// 在新的IpAddr枚举定义中，V4和V6两个变体都被关联上了一个String值：

enum IpAddr_str {
    V4(String),
    V6(String),
}


//另外一个使用枚举代替结构体的优势在于：每个变体可以拥有不同类型和数量的关联数据。
enum IpAddr_ {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
      // 方法体可以在这里定义
    }
}




fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // 使用结构体解决问题
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // 我们直接将数据附加到了枚举的每个变体中，这样便不需要额外地使用结构体。
    let home = IpAddr_str::V4(String::from("127.0.0.1"));

    let loopback = IpAddr_str::V6(String::from("::1"));

    let home = IpAddr_::V4(127, 0, 0, 1);

    let loopback = IpAddr_::V6(String::from("::1"));


    let m = Message::Write(String::from("hello"));
    m.call();

}

//枚举的变体全都位于其标识符的命名空间中，并使用两个冒号来将标识符和变体分隔开来
fn route(ip_type: IpAddrKind) { }