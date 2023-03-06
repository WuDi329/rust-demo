use std::collections::HashMap;
fn main() {
    let mut table = HashMap::new();
    table.insert("Sally", "项目");
    table.insert("Ahn", "项目");
    table.insert("Amir", "销售");
    table.insert("Tom", "财务");
    table.insert("Jerry", "人事");
    table.insert("Jack", "人事");
    sort_hashmap(&table);
    println!("{:?}", table);

    get_all_department(&table);

    get_specific_department(&table, "人事");
}

fn get_all_department(table: &HashMap<&str, &str>){
    for (key, value) in table {
        println!("{}-{}", key, value);
    }
}

fn sort_hashmap(table: &HashMap<&str, &str>){
    let mut hash_vec: Vec<(&&str, &&str)> = table.iter().collect();
    println!("{:?}", hash_vec);
    hash_vec.sort_by(|a, b| a.0.cmp(b.0));
}

fn get_specific_department(table: & HashMap<&str, &str>, dep_name: &str){
    for (key, value) in table {
        if *value == dep_name {
            println!("{}-{}", key, value);
        }
    }
}



