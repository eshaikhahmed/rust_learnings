
fn main() {

    //bit new way of initializing array 
    let xs: [i32; 4] = [5,10,40,4];
    
    //simple function to count total no. of elements
    let size = xs.len();
    
    println!("first element of the array: {} & no. elements {}", xs[0], size);

    //Awesome!!, do you get it ?? Yup it is looking weird but
    //Trust me it is awesome
    // we just sliced the part of array from index 1 to 3 (so slices will have [10,40] not 0) 
    let slice: &[i32] = &xs[1..3];
    println!("first element of the array: {}", slice[1]);
    
    //slice[0] = 10; //This is not allowed, you cannot change the value of slice
}









