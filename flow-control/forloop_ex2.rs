 fn main(){
     let great_minds = vec!["Andre", "Philippe"];

     for name in great_minds.iter() {
         println!("Happy New Year {}", name);
     }

     //iter doesn't consume and available to re-use
     println!("{:?}", great_minds);
     
     for name in great_minds.into_iter() {
         println!("Enjoy your new year with smile {}", name);
     }

     //into_iter consume all elements in a way that , it is not longer available to reuse.
     //println!("{:?}", great_minds);
 }








 