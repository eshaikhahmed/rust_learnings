use std::fmt;

struct Person{
    age: i32
}


//This is an implementation of formatter for Person structure
//It is embedded with to_string, whether we call to string or not, 
//It works perfectly.
impl fmt::Display for Person {

    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "My age is {}", self.age)
    }

}

fn main(){
    
    let person = Person{age: 40};
    
    //we haven't called to string
    println!("{}", person);

    //Integer variable to string
    let year: i32 = 2022;
    let yearS: String = year.to_string();
    println!("Happy New Year {}", yearS);

    //Float variable to string
    let angerFloat: f32 = 25.25;
    println!("Amazing {}", angerFloat.to_string());


}


