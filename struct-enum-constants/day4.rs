
//We need to display or print whole structure , that's why debug
#[derive(Debug)]
struct Point{
    x: f32,
    y: f32,
}

// duplicate structure name is not allowed
#[derive(Debug)]
struct Point2(i32, f32);

fn main(){

    let first_point = Point{x: 2.3, y: 4.5};
    println!("{:?}", first_point);
    
    let second_point = Point2(2,1.4);
    println!("{:?}", second_point);
}