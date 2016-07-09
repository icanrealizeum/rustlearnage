struct Foo<'a> {
    x: &'a i32,
}

fn main() {
        let y = &5;           // ---+ y goes into scope
    let x;                    // -+ x goes into scope
                              //  |
    {                         //  |
        let f = Foo { x: y }; // ---+ f goes into scope 'can use &y too here'
//        let f = Foo { x: &y }; // ---+ f goes into scope 'can use &y too here' - same effect
        x = &f.x;             //  | | error here; but works with just 'y'
//        x = y;
    }                         // ---+ f goes out of scope
                              //  |
    println!("{}", x);        //  |
}                             // -+ x and y go out of scope

