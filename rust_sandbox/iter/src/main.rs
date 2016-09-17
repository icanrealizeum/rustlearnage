#![feature(inclusive_range_syntax)]
//src: https://github.com/rustomax/rust-iterators#digging-deeper

fn main() {
    for j in 1...3 {
        for i in (1..11).map(|x| x +j) {
            print!("{} ", i);
        }
        println!("");
    }

    let v = vec![3, 5, 0, -2, 3, 4, 1];
    //    let v:Vec<i32> = vec![];
    //  let v:i32 = vec![];
    let max = v.iter().max();
    let min = v.iter().min();

    println!("max = {:?}, min = {:?}", max, min);

    // output: max = Some(5), min = Some(-2)

    // pass the starting temperature and step to the initializer function
    let ftc = FahrToCelc::new(0.0, 5.0);

    // produce the iterator table of first 5 values
    let temp_table = ftc.take(5);

    // print out the temperature table nicely
    for (f, c) in temp_table {
        println!("{:7.2} °F = {:7.2} °C", f, c);
    }
}

struct FahrToCelc {
    fahr: f32,
    step: f32,
}

impl FahrToCelc {
    fn new(fahr: f32, step: f32) -> FahrToCelc {
        FahrToCelc { fahr: fahr, step: step }
    }
}

impl Iterator for FahrToCelc {
    type Item = (f32, f32);

    fn next (&mut self) -> Option<(f32, f32)> {
        let curr_fahr = self.fahr;
        let curr_celc = (self.fahr - 32.0) / 1.8;
        self.fahr = self.fahr + self.step;
        Some((curr_fahr, curr_celc))
    }
}

