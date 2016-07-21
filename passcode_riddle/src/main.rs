// can you solve the passcode riddle https://www.youtube.com/watch?v=7Vd1dTBVbFg

fn main() {
    for x in 1..9 {
        for y in 1..9 {
            for z in 1..9 {
                if (x<=y)&&(y<=z)&&(z != y)&&(x*y*z==36) {
                    println!("{} {} {} sum={}",x,y,z,x+y+z);
                }
            }
        }
    }
}
