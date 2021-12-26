fn main(){
    let mut new_year = 2022;
    println!("Welcome {0}" , new_year);
    
    {
        //shadow copy
        let new_year = new_year;
        
        //Not allowed, so value is freezed here in entire block
        //new_year = 2023;
    }
    
    new_year = 2023; //allowed because of there is no shadow copy
    println!("Welcome {0}" , new_year);
}