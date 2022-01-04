
fn main(){
    print_numbers(5);
}

//Function returns unit, also act similar as void function
fn print_numbers(n: i32) -> () {
    for val in 1..=n {
        println!("Welcome to simple {}", val);
        if is_even_number(val) {
            println!("Number even");
        } else {
            println!("Number odd");
        }
    }
}

//fucntion return bool
fn is_even_number(val: i32) -> bool{
    if val % 2 == 0 {
        return true
    } else {
        return false
    }

}