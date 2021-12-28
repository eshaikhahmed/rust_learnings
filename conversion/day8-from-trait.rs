use std::convert::From;

#[derive(Debug)]
struct Number{
    value: i32
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}


fn main(){
    //This way, we don't have to worry about complex struct sytax like  Number { value : 25}
    //One time complexity will help us fast forward in our own framework
    let age = Number::from(48);
    println!("My age is {:?} ", age);

    let oldAge = 25;
    
    //simply convert integer into Number structure.
    let oldAgeNum: Number = oldAge.into();
    println!("My number is {:?}", oldAgeNum);
}

