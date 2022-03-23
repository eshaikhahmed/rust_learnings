fn main() {
    let mut name = String::from("Hello");
    name.push_str(" World!");
    println!(" {}", name);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    
    let concat_str = s1 + &s2 + &s3;

    //s2 is moved so it cannot be accessible.
    println!("{}", concat_str);

    let welcome = String::from("hello");
    let count = welcome.chars().count();
    println!("Total hello length: {}", count);
}
