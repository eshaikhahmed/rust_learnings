
fn main(){
    let number = 45;
    match number {
        //normal match
        1 => println!("Number is 1"),

        //range matching
        1 | 2 | 45 => println!("Number is 45"),
        _ => println!("Some other nummber"),

    }
}