#[derive(Debug)]
struct Foo{
a: i32
}

fn main() {
    //let &mut z=&mut Foo{a:1};//lol?
//    let mut z: Foo= Foo{a:1};
    let mut z = Foo{a:1};
    let mut y = Foo{a:2};
    let p: Foo= Foo{a:3};
	{
    //let x;
    //let &x;
    //let mut x;//XXX: same effect as the above! - NOPE! now the binding x is mutable! can re-assign x!
    //let &mut x;//XXX: same effect as the above! - NOPE different that let mut x above! cannot re-assign x!
    let &mut mut x;//XXX: - now can re-assign x!
    //let x:&mut Foo;//this makes sense! and is different
    //ok it's all explained here: https://doc.rust-lang.org/nightly/book/mutability.html

    x = &mut z;
    x.a=41;
	x= &mut y;
    //x=p;//moved here

    //let x:&mut Foo=&mut Foo{a:1};
    x.a=42;
    println!("x={:?}", x);
	}
    println!("z={:?}", z);
    println!("y={:?}", y);
}

