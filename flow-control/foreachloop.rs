
fn main(){

    let source = vec![1,2,3]; 
    let mut target = vec![1,2,3]; 
    target.iter_mut().zip(source.iter()).for_each(|(t, s)| *t += s);

    println!("{:?}", target);
}