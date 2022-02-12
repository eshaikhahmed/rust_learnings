
fn main() {
     //we've created mutated string
     let mut your_name = String::new();

     println!("Enter your name: ");

     //read input from user
     std::io::stdin().read_line(&mut your_name).unwrap();

     //This write function will create a file if it does not exist, 
     //and will entirely replace its contents if it does.
     std::fs::write("rust_db.txt", your_name);

}

