fn main(){
    
    //new and empty vector, but it has no idea which values will it have rn.
    let mut vec = Vec::new();

    //now compile knows its Vec<u8>
    vec.push(5u8); 

    println!("{:?}", vec);

}




