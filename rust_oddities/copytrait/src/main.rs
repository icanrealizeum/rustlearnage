#![feature(plugin)]

#![plugin(clippy)]


#![deny(clippy)]
#![deny(clippy_pedantic)]
#![allow(print_stdout)]
#![allow(clone_on_copy)]

fn main() {
    let mut a = [1, 2, 3, 4, 5];

    //implicity Copy trait - the bane of Rust :)
    let first = a[0]; //can I prevent this Copy from happening? a lint for it? eg. I actually wanted &a[0] or a move
    let oldfirst = a[0].clone();//this is acceptable! but implicitly copy like above isn't! TODO: find a way to disable Copy trait!

    a[0] = 7;
    let reffirst:&i32 = &a[0]; //TODO: find out how to keep a ref and still allow assignment - eg. move this line above the assignment line a[0]=7; and have it work!

    println!("The value of first is: {} {} {}", first, oldfirst, reffirst);
}
