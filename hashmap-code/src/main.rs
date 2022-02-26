use std::collections::HashMap;

fn main() {
    let mut names = HashMap::new();

    //Add hashmap key and values.
    names.insert("Phillip", "Amazing person I've met on LinkedIn");
    
    match names.get("Phillip") {
        Some(mark) => println!("{}", mark) ,
        None => println!("I suggest you should meet")
    };

}
