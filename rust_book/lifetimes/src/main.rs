struct Foo<'a> {
    x: &'a i32,
}

fn main() {
    let _y=5;
    let x;                    // -+ x goes into scope
                              //  |
    {                         //  |
        let y = &_y;           // ---+ y goes into scope
        let f = Foo { x: y }; // ---+ f goes into scope
        x = *&f.x;             //  | | error here
    }                         // ---+ f and y go out of scope
                              //  |
    println!("{}", x);        //  |
}                             // -+ x goes out of scope

