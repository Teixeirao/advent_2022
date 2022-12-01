use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut cvalue = 0;
    let mut max = 0;
    let mut max0 = 0;
    let mut max1 = 0;
    let mut max2 = 0;
    for line in reader.lines() {
        if let Ok(l) = line {
            if l.is_empty() {
                println!("cvalue {cvalue}");
                if cvalue > max {
                    max = cvalue;
                }
                if cvalue > max0 {
                    max2 = max1;
                    max1 = max0;
                    max0 = cvalue;
                } else if cvalue > max1 {
                    max2 = max1;
                    max1 = cvalue;
                } else if cvalue > max2 {
                    max2 = cvalue;
                }
                
                cvalue = 0;
            } else {
                let my_int = l.parse::<i32>().unwrap();
                cvalue += my_int;
            }
        }
    }
    println!("max {max}");
    let maxsum = max0 + max1 + max2;
    println!("max3 {maxsum}");

    Ok(())
}