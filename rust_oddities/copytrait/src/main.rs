#![feature(plugin)]

#![plugin(clippy)]


#![deny(clippy)]
#![deny(clippy_pedantic)]
#![allow(print_stdout)]
#![allow(clone_on_copy)]
#![allow(many_single_char_names)]

#![allow(use_debug)]

#[derive(Debug)]
struct Moo{
    a:i32, //I guess no one else can access these fields(unless marked pub) except me from within here
    b:i32,
    c:String
}

fn main() {
    let mut a = [1, 2, 3, 4, 5];

    //implicit copying OR moving - the bane of Rust :)
    let first = a[0]; //can I prevent this Copy from happening? a lint for it? eg. I actually wanted &a[0] or a move
    let oldfirst = a[0].clone();//this is acceptable! but implicitly copy like above isn't! TODO: find a way to disable Copy trait!

    a[0] = 7;
    let reffirst:&i32 = &a[0]; //TODO: find out how to keep a ref and still allow assignment - eg. move this line above the assignment line a[0]=7; and have it work!

    println!("The value of first is: {} {} {}", first, oldfirst, reffirst);

    //same syntax can do either an implicity copy or an implicit move - i'd rather have one or the
    //other! if not at least warn me?
    let a1 = [1,2,3];
    let first1 = a[0]; // implicit copy
    let second = a[1];
    let b = [String::from("1"),String::from("2"),String::from("3")];
//    let fb=b[0]; // this wants to do an implicit move
//    let sb=b[1];

    let meh = String::from("meh");
    let mut x=Moo{a:1,b:2,c:meh};
//    println!("{}",meh);//meh was moved! awesome! i approve
//    let y=x;//moved!
    let y=x.a;//copy!
    x.a=3;//bye bye stale value y, right now.
    println!("{:?} {}",x,y);

    let z=&x.a;//borrowed! eg.temp moved!
//    x.a=4;//can't do this! this is great!
//    TODO: how do I do explicit move tho?
    println!("{:?} {}",x,y);
    {
        let mut q=x.c;//implicitly moved.
    //    println!("{:?}",x);//can't do this: use of partially moved value!
        q.push_str("something");
    //    but can still access the other fields meanwhile. hmm...
        let somethingelse=x.a;//hmm, maybe this isn't as bad as I thought...or is it? I'm unsure... will get back to this after I know more!
        x.c=q;//move it back!
//        x.c=String::from("meh2");//or, assign new stuff instead - so it's not partially moved
//        value anymore!
    }//block has no effect, I just have to move it back manually! obv.
    println!("{:?}",x);
}
