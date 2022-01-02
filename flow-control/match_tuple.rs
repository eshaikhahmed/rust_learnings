
fn main(){
    let tuple22 = (6,2,4);

    match tuple22 {
        (1, two, four) => println!("Tuple should start with 1, remaining are just variables {} {}", two, four),

        //starts with something , rest ignored
        (6, ..) => println!("Startswith 1 and rest ignored"),

        _ => println!("Nothing man"),
    }
}