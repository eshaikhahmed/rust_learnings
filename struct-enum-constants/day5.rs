
#[derive(Debug)]
enum Rustlearning{
    HOME,
    STUDENT,
}

fn main(){

    use crate::Rustlearning::*;
    
    let status = HOME;
    println!("{:#?}", status);

}