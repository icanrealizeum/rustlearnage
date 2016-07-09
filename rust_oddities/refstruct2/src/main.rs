struct Foo<'a> {
    x: &'a i32,
}

fn main() {
    let _y=5;
    let y = &_y;

//    let z: &i32;//works! see: XXX:
//    let &z;//doesn't work! see: XXX:
    let z;//doesn't work! see: XXX:

    {
        let f:Foo;

        //let &x;//same thing?
        let x;//same thing?

        {
            f = Foo { x: y };
        }
        x = f.x;  //this works! because it's a ref to i32
//        x = &f.x;  //XXX: error here - oh, it's a ref to a ref!
        z = x; //so z should be y here, right?
    }
    println!("{}", z);
}

