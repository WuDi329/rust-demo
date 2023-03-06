use std::hash::Hash;

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 另外一个构建哈希映射的方法是，在一个由键值对组成的元组动态数组上使用collect方法。
    // use std::collections::HashMap;

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // 那么我们就可以使用zip方法来创建一个元组的数组，其中第一个元组由"Blue"与10组成，以此类推。
    // 接着，我们还可以使用collect方法来将动态数组转换为哈希映射
    // 对于那些实现了Copy trait的类型，例如i32，它们的值会被简单地复制到哈希映射中。
    // 而对于String这种持有所有权的值，其值将会转移且所有权会转移给哈希映射，如示例8-22所示。
    let scores: HashMap<_, _> =
    teams.iter().zip(initial_scores.iter()).collect();

    // collect将迭代器转换为集合。
    let score: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    //可以通过将键传入get方法来获得哈希映射中的值
    // use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // 上面这段代码中的score将会是与蓝队相关联的值，也就是Some(&10)。因为get返回的是一个Option<&V>，所以这里的结果被封装到了Some中
    let score = scores.get(&team_name);

    if let Some(s) = score {
        println!("{}", s);
    }else {
        println!("none");
    }

    // use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 想要修改哈希映射中的数据时，你必须要处理某些键已经被关联到值的情况
    // use std::collections::HashMap;

    // 覆盖旧值
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);


    // 在实际工作中，我们常常需要检测一个键是否存在对应值，如果不存在，则为它插入一个值。
    // 哈希映射中提供了一个被称为entry的专用API来处理这种情形，它接收我们想要检测的键作为参数，并返回一个叫作Entry的枚举作为结果。
    // 这个枚举指明了键所对应的值是否存在。比如，我们想要分别检查黄队、蓝队是否拥有一个关联的分数值，如果该分数值不存在，就将50作为初始值插入。

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // Entry的or_insert方法被定义为返回一个Entry键所指向值的可变引用，假如这个值不存在，就将参数作为新值插入哈希映射中，并把这个新值的可变引用返回。
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);


    // use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
