
fn main() {
    let awesome_variable = 2;

    //This is not allowed, cannot change as it is immutable
    //awesome_variable = 5;

    //This you can do, copy variable into same or other variable.
    let awesome_variable = awesome_variable + 1;
    
    println!("{}", awesome_variable);
}

