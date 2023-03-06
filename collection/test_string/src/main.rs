use std::vec;

fn main() {
    let strs = vec![String::from("value"), String::from("sad"), String::from("umbrella"), String::from("happy")];
    for i  in strs{
        println!("{}", change(i));
    }
}

fn change(str: String) -> String{
    let yuan = vec!['a','e', 'i', 'o', 'u'];
    for i in yuan {
        if str.starts_with(i) {
            return str + "-hay";
        }
    }
    let firstletter = &str[0..1];
    let otherletter = &str[1..].to_string();
    return format!("{}-{}{}",otherletter, firstletter, "ay");
}
