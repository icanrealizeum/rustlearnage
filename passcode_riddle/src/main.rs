// can you solve the passcode riddle https://www.youtube.com/watch?v=7Vd1dTBVbFg

//use std::cmp::Ord;
//use std::cmp::Eq;
use std::cmp::Ordering;

#[derive(Debug,Clone)]
struct Moo {
    x:u8,
    y:u8,
    z:u8,
    sum:u16,
}

impl std::cmp::PartialEq for Moo {
    fn eq(&self, other: &Self) -> bool {
        if (self.sum == other.sum)&&(self.x == other.x)&&(self.y==other.y)&&(self.z==other.z) {
            return true;
        } else {
            return false;
        }
    }
}

impl std::cmp::Eq for Moo {

}

impl std::cmp::PartialOrd for Moo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.sum > other.sum {
            return Option::Some(Ordering::Greater);
        }else if self.sum < other.sum {
            return Option::Some(Ordering::Less);
        } else {
            return Option::Some(Ordering::Equal);
        }
    }
}

impl std::cmp::Ord for Moo {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.sum < other.sum {
            return Ordering::Less;
        }else if self.sum > other.sum {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    }
}

fn main() {
    const MAX:u8 = 9;
    //casting to different types! how nice is that ;-)
    //println!("{}",1u8 as u16+1u16 as u16);
//    let x:u16;
    let mut candidates: Vec<Moo>=vec![];
    for x in 1..1+MAX {//(1+MAX as u16) {
        for y in 1..MAX+1 { //1u16+MAX as u16 {
            for z in 1..MAX+1 {//1+(MAX as u16) {
                if (x<=y)&&(y<=z)
//                    &&(z != y) not yet!
                        && (x as u16 * y as u16 * z as u16) as u16 == 36 //clue1
                        {
                            let m:Moo = Moo {x:x, y:y,z:z,sum:(x+y+z) as u16};
                            candidates.push(m);//clue2 - storing all sums, for now
                    println!("{} {} {} sum={}",x,y,z,x+y+z);
                } else {
//                    println!("{} {} {} rejected",x,y,z);
                }
            }
        }
    }
    //println!("{}",x);
    candidates.sort();
    println!("Candidates(clue1): {:?}",candidates);
    //should keep only dups, because that's why she's unsure - and needed clue3
    //irrelevant block, ignore:
/*    let mut prev=0;
    for i in candidates.iter() {
        if (i.z != i.y) {//clue3 - largest number appears only once
            println!("{:?} clue3 ok", i);
            if prev == i.sum {
                println!("dup: {:?}", i);//coincidence that 229 is last!
            }
        }
        prev=i.sum;
    }*/

    let mut cand2:Vec<Moo> = Vec::new();
    for i in 0..candidates.len()-1 {
        if candidates[i].sum == candidates[i+1].sum {
            cand2.push(candidates[i].clone());
            cand2.push(candidates[i+1].clone());
        }
    }
    println!("Candidates(after clue2): {:?}", cand2);//when she's unsure
    for i in cand2.iter() {
        if i.z != i.y {//clue3 - largest number appears only once
            println!("Found valid passcode {:?}", i);
        }
    }


    /*for x in 1..1+MAX {
        for y in 1..1+MAX {
            for z in 1..1+MAX {
                
            }
        }
    }*/
}
