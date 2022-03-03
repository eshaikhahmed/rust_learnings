use std::collections::HashMap;

fn main() {
    let mut names = HashMap::new();
    
    //One of LinkedIn my friend(Bastian) suggested this approach
    names.entry("Make is slow and short.").or_insert("Easy to consume!");

    for (key, value) in names.into_iter() {
        println!("{} - {}", key, value);
    }
}


