// can you solve the passcode riddle https://www.youtube.com/watch?v=7Vd1dTBVbFg
fn main() {
    const MAX:u8 = 9;
    //casting to different types! how nice is that ;-)
    //println!("{}",1u8 as u16+1u16 as u16);
//    let x:u16;
    for x in 1..(1+MAX as u16) {
        for y in 1..1u16+MAX as u16 {
            for z in 1..1+(MAX as u16) {
                if (x<=y)&&(y<=z)&&(z != y)&&(x*y*z==36) {
                    println!("{} {} {} sum={}",x,y,z,x+y+z);
                } else {
//                    println!("{} {} {} rejected",x,y,z);
                }
            }
        }
    }
    //println!("{}",x);
}
