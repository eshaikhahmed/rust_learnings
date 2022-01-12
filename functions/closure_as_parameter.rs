fn apply<F>(f: F)
where
    F: Fn(),
{
    println!("Yup! it is advantageous in terms of many things");
    println!(
        "I would say many functions as arguments within single function"
    );

    //This is a closure call here
    f();
}

fn main() {
    let year = 2022;

    let print = || println!("{}", year);

    //Function call here
    apply(print);
}
