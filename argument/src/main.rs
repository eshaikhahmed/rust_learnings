fn main() {
    //skips first file parameter
    //return iterator
    let mut args = std::env::args().skip(1);

    //next() capture value , unwrap breaks our program
    //if next() function doesn't return anything
    let value1 = args.next().unwrap();

    let value2 = args.next().unwrap();

    println!("Welcome to your 1st arg {} 2nd arg {}", value1, value2);
}
