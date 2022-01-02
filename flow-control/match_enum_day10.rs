enum Person{
    Home,
    Token(i32, i32, i32),
}

fn main() {

    // let person = Person::Token(10, 20, 30);
    let person = Person::Home;

    match person {

        // enum bases token
        
        Person::Token(x,y,z) => println!("Hello World"),
        
        _ => println!("Amazing"),
    }


}