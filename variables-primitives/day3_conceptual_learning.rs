//It is thanks to Andre for clearing my doubts on this example


fn main() {
let x:[i32; 3] = [1, 2, 3]; //immutable

// we can create many as such , immutable array can borrow as many mutable slices as we want ? yes
let slice =  &x[0..2];
let slice1 = &x[1..2];

// immutable array cannot borrow the slice of mutable ? yup
//let slice2 = &mut x[0..2]; 

/*
Not exactly: You could make `x` mutable with
`let mut x = [1,2,3];` 
then `let slice2 = &mut x[0..2]` would work.
 However, 
 you cannot borrow both `slice2` and `slice` or `slice1` 
 at the same time.

*/

/*
Yes. `slice` and `slice1` can be used at the same time without limitation. 
`slice2` would need making `x` mutable
 (because you cannot borrow an immutable value mutably),
  and as it represents a mutable borrow, 
  you could not have any further uses of `slice` nor `slice1` 
  in this scope as long as `slice2` is alive.
/*
}