fn main() {
    //we've created mutated string
    let mut your_name = String::new();

    println!("Enter your name: ");
    
    //read input from user
    std::io::stdin().read_line(&mut your_name).unwrap();

    println!("Welcome to Rust {} ", your_name);
}

/*

> cargo new input-user
> cargo build
> cargo run


*/
