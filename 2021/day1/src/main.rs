use std::io::{self, BufRead};

fn main() {

    let mut lastval: i32 = -6666;
    let mut positive: i32 = 0;

    let mut lastsum: i32 = -6666;
    let mut sumpositive: i32 = 0;
    let mut a: i32 = -6666;
    let mut b: i32 = -6666;
    let mut c: i32 = -6666;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let readval: i32 = match line.unwrap().trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        if lastval < -1000 {
            lastval = readval;
        }

        let diff: i32;
        diff = readval - lastval;

        //println!("{} {} {}", lastval, readval, diff);

        if diff > 0 {
            positive = positive + 1;
        }
 
        lastval = readval;




        if c < -1000 {
            c = readval;
            continue;
        }
        if b < -1000 {
            b = c;
            c = readval;
            continue;
        }
        if a < -1000 {
            a = b;
            b = c;
            c = readval;
            lastsum = a + b + c;
            continue;
        }

        let newsum: i32 = b + c + readval;
        if newsum > lastsum {
            sumpositive = sumpositive + 1;
        }

        a = b;
        b = c;
        c = readval;
        lastsum = newsum;

    }
    println!("positive {}", positive);
    println!("sumpositive {}", sumpositive);
}
