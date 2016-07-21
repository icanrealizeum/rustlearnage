// can you solve the passcode riddle https://www.youtube.com/watch?v=7Vd1dTBVbFg
//uhm, why am I not getting 2 2 9?
fn main() {
    for x in 1..10 {
        for y in 1..10 {
            for z in 1..10 {
                if (x<=y)&&(y<=z)&&(z != y)&&(x*y*z==36) {
                    println!("{} {} {} sum={}",x,y,z,x+y+z);
                } else {
//                    println!("{} {} {} rejected",x,y,z);
                }
            }
        }
    }
}
