fn main(){
    let x = 5;
    let y = {
        let square = x*x;
        let multiple = square + x;

        //no semicolor, this value will get assinged
        x+square+x
    };

    println!("Welcome new year value {} {}", y, x);

}