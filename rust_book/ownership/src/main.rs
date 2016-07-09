//src: file:///usr/share/doc/rust/html/book/ownership.html
use std::rc::Rc;

struct Car {
    name: String,
}

struct Wheel {
    size: i32,
    owner: Rc<Car>,
}

fn main() {
    println!("Hello, world!");

    //ownership:
    let x = Box::new(5);
    let y = add_one(x);
    println!("{}", y);


    //borrowing:
    let mut x = 5;
    add_one_borrows_arg(&mut x);
    add_one_borrows_arg(&mut x);
    println!("{}", x);

    //lifetimes:
    {
        //        let x;                    // -+ x goes into scope
        //  |
        {                         //  |
            let y = &5;           // ---+ y goes into scope
            let f = Foo { x: y }; // ---+ f goes into scope
            //            x = &f.x;             //  | | error here
        }                         // ---+ f and y go out of scope
        //  |
        //        println!("{}", x);        //  |
    }  // -+ x goes out of scope

    //The lifetime named static is a special lifetime. It signals that something has the lifetime
    //of the entire program. Most Rust programmers first come across 'static when dealing with
    //strings:

    let x: &'static str = "Hello, world.";
    static FOO: i32 = 5;
    let x: &'static i32 = &FOO;

    //shared ownership: // This is the simplest kind of multiple ownership possible.
    let car = Car { name: "DeLorean".to_string() };
    let car_owner = Rc::new(car);
    for _ in 0..4 {
        Wheel { size: 360, owner: car_owner.clone() };
    }

    //elided lifetimes: see: file:///usr/share/doc/rust/html/book/ownership.html#examples
}//main

struct Foo<'a> {
    x: &'a i32,
}

//This function takes ownership, because it takes a Box, which owns its contents. But then we give ownership right back.
fn add_one(mut num: Box<i32>) -> Box<i32> {
    *num += 1;
    //To fix this, we can have add_one give ownership back when it's done with the box:
    num
        // "we return a(the same?u mean) box, and so the ownership is transferred back to y in main."
        //"We only have ownership for the duration of our function before giving it back. "
}


//This function borrows an i32 from its caller, and then increments it. When the function is over, and num goes out of scope, the borrow is over.
fn add_one_borrows_arg(num: &mut i32) {
    *num += 1;
}
