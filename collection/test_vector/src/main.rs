use std::collections::HashMap;
fn main() {
    let mut arr = vec![2,5,13,42,1,31,13,1,4,413,13,12,13,41, -1, 10,2];
    let len = arr.len();
    sort(&mut arr);
    println!("{:?}", arr);
    println!("average of arr is {}", get_average(&arr));
    println!("zhogn of arr is {}", get_most(&arr));
    println!("middle of arr is {}", get_middle(&arr));


}
fn get_middle(arr: &Vec<i32>) -> i32{
    let mut len = arr.len();
    len /= 2;
    return arr[len];
}

fn sort(arr: &mut Vec<i32>) {
    arr.sort();
}

fn get_most(arr: &Vec<i32>) -> i32{
    let mut count = HashMap::new();
    let mut most = 0;
    let mut zhong = 0;
    for i in arr{
        let cur = count.entry(i).or_insert(0);
        *cur += 1;
        if *cur > most {
            most = *cur;
            zhong = *i;
        }
    }
    return zhong;

}

fn get_average(arr: &Vec<i32>) -> i32{
    let mut sum = 0;
    for i in arr{
        sum += i;
    }
    return sum / (arr.len()) as i32;
}