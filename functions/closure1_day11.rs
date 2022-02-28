
fn main(){

    //Function
    fn welcome() -> i32 {  return 2022 };

    //close without parameter
    let firstClosure = || -> i32 { return 2022 };
    
    println!("Welcome {} ", welcome());
    println!("Welcome {} ", firstClosure());

    let awesome_value = 12; 
    //closure with parameter
    let secondClosure = |i| -> i32 { return i*awesome_value };

    println!("Second closure {} ", secondClosure(2));
}