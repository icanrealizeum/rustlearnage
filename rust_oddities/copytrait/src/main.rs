#![feature(plugin)]

#![plugin(clippy)]


#![deny(clippy)]
#![deny(clippy_pedantic)]
#![allow(print_stdout)]

fn main() {
    let mut a = [1, 2, 3, 4, 5];

    let first = a[0]; //can I prevent this from happening? a lint for it? eg. I actually wanted &a[0]

    a[0] = 7;

    println!("The value of first is: {}", first);
}
