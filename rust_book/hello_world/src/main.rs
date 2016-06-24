#![warn(missing_docs)]
#![allow(dead_code)]
#![allow(unused_variables)]

//the following is the crate documentation, if you get missing documentation for crate
//! Tools for dealing with universes (this is a doc comment, and is shown on
//! the crate index page. The ! makes it apply to the parent of the comment,
//! rather than what follows).


//fn main() -> i32 { // cannot be done, RFC: https://github.com/rust-lang/rfcs/issues/1176
fn main() { // main must have this format: https://doc.rust-lang.org/error-index.html#E0211
    println!("Hello, world!");
    println!(
        "Hello, world!"
        );
    println!
        (
        "Hello, world!"
        )
        ;
//bindings:
    let x = 5;//type inference
    let x: i32 = 5;//explicitly state the type; "x is a binding with the type i32 and the value five."
    let (x, y) = (1, 2);//the left hand side of a let expression is a full pattern, not just a variable name.
    let x=6;//allowed
//    x=7;//not! error: re-assignment of immutable variable `x`

    //If you want a binding to be mutable, you can use mut:
    let mut x = 5; // mut x: i32
    x=10;//now allowed
//nice warnings above ^  love this error reporting stuff!

    let x: i32;
//    let y;//won't work
//    excellent error reporting(love it!):
//    println!("The values {} {}",x,x);
//    println!("The values {} {}",x,x,x);
    x=1;
    println!("The value {}",x);
    let y: i32 = 2;
    println!("The values {} {}",x,y);

//if:
    let x = 5;
    if x == 5 {
        println!("x is five!");
    } else {
        println!("x is not five :(");
    }


    let y = if x == 5 {
        10
    } else {
        15
    }; // y: i32

    let y = if x == 5 { 10 } else { 15 }; // y: i32
    // Rust is primarily an expression based language. 
    println!("y={}",y);

//    let z=let y=1; yep ridiculous
//    let z=y=1;//works but no idea what it does currently
//    println!("z={}",z);//the trait `core::fmt::String` is not implemented for the type `()`
    //in Rust the value of an assignment is the unit type () 
    //so that's why z=()  because y=1 returns ()

//    let y: i32 = if x == 5 { 10; } else { 15; };//sneaky - without that i32
    let y = if x == 5 { 10; } else { 15; };//sneaky - without that i32 y=() !
    // () is pronounced: unit

//functions:
    print_number(x);
    print_number(101);
    print_sum(1,2);
    print_number(add_one(1));
    deadcodetest1(1);
    deadcodetest2(1);
    foo(1);


//tuples:
    let x = (1, "hello");
    let x: (i32, &str) = (1, "hello");
    let (x,y) = x;//x gets overwritten with 1
    println!("{}",x);
    let (x, y, z) = (4, 2, 3); //destructuring let
    println!("x is {}", x);

    let mut x = (1, 2); // x: (i32, i32)
    let y = (2, 3); // y: (i32, i32)
    x = y;
//    println!("y is {}", y);
    let x = (1, 2, 3);
    let y = (2, 2, 4);

    if x == y {
            println!("yes");
    } else {
            println!("no");
    }

    let (x, y) = next_two(5);
    println!("x, y = {}, {}", x, y);

//structs:
    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 }; // origin: Point
//    origin.x=1;//error: cannot assign to immutable field `origin.x`
    println!("The origin is at ({}, {})", origin.x, origin.y);

    //mutable values in struct
    let mut point = Point { x: 0, y: 0 };
    point.x = 5; //can do
    println!("The point is at ({}, {})", point.x, point.y);

//tuple structs:
    struct Color(i32, i32, i32);
    struct Point2(i32, i32, i32);
    //These two will not be equal, even if they have the same values:
    let black = Color(0, 0, 0);
    let origin = Point2(0, 0, 0);

//newtype:
    struct Inches(i32);
    let length = Inches(10);
    let Inches(integer_length) = length;//destructuring
    println!("length is {} inches", integer_length);
//enum
    enum Ordering {
        Less,
        Equal,
        Greater,
    }

    for x in 0..10 {
        //The upper bound is exclusive, though, so our loop will print 0 through 9, not 10.
        println!("{}", x); // x: i32
    }

    let mut x = 5; // mut x: u32
    let mut done = false; // mut done: bool
    while !done {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 { done = true; }
    }

    let mut x = 5;
    loop {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 { break; }
    }

//    for x in range(0,10) { //src/main.rs:158:14: 158:19 error: unresolved name `range` [E0425] // XXX: this is also NOT detected by rustc -Z parse-only  the reason is obvious, but if not, see this for explanation: https://github.com/scrooloose/syntastic/commit/a39b397e6ae3b32d2a0497d0f4ddfec0479ad2bb#commitcomment-6652907   It's detected by -Z no-trans
    for x in 0..10 {
        if x % 2 == 0 { continue; }
        println!("{}", x);
    }

    std::process::exit(2);//caveats(doesn't execute Drop): https://doc.rust-lang.org/std/process/fn.exit.html
}//main


fn next_two(x: i32) -> (i32, i32) { (x + 1, x + 2) }

fn deadcodetest1(x: i32) -> i32 {
    if x < 5 {
        return x;
    } else {
        return x + 1;
    }
    x + 2 //warning: unreachable expression, #[warn(unreachable_code)] on by default
}

fn deadcodetest2(x: i32) -> i32 {
    if x < 5 {
        x
    } else {
        x + 1
    }
//    x + 2 //error below if this line isn't commented out
/*
error isn't obvious, but still better than none!

src/main.rs:82:9: 82:10 error: mismatched types:
 expected `()`,
    found `i32`
(expected (),
    found i32)
src/main.rs:82         x
                       ^
src/main.rs:84:9: 84:14 error: mismatched types:
 expected `()`,
    found `i32`
(expected (),
    found i32)
src/main.rs:84         x + 1
                       ^~~~~
*/
}

fn foo(x: i32) -> i32 {
    if x < 5 {
        x
    } else {
        x + 1
    }
//    return 1;
}


fn add_one(x: i32) -> i32 {
    x + 1  // <- note the lack of ; at the end! which would otherwise return () aka unit
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

#[warn(missing_docs)] //<- no effect on this function, only on main! 
fn print_sum(x: i32, y:i32) {
    println!("sum: {}", x+y);
}

#[warn(missing_docs)] //<- no effect on this function, only on main! 
/// `hello` is a function that prints a greeting that is personalized based on
/// the name given.
///
/// # Arguments
///
/// * `name` - The name of the person you'd like to greet.
///
/// # Example
///
/// ```rust
/// let name = "Steve";
/// hello(name); // prints "Hello, Steve!"
/// ```
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

