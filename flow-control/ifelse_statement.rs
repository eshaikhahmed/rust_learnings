fn main(){
    let year = 2022;
    if year == 2021 {
        println!("Past year");
    } else if year == 2022 {
        println!("Happy New Year 2022");
    } else {
        println!("All the best!");
    }

    //Cleaner way of using if else for assignment
    let value = if year > 2021 {
        "Happy New Year"
    } else {
        "Old Year"
    };

    println!("{}", value);
}