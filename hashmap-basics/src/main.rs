use std::collections::HashMap;

fn main() {
    let mut names = HashMap::new();

    names.insert("1", "Its monday!!");
    names.insert("2", "I feel lazy :(");
    println!("Let's see: {:?}", names.get("1"));
    if names.contains_key("1") {
        println!("Yes! Today is monday");
    } 
    names.remove("2");
    println!("We've removed '2', we will get {:?}", names.get("2"));
}


