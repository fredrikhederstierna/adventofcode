
use std::io::{self, BufRead};

fn main() {

    let mut numbers = String::new();

    let stdin = io::stdin();

    for line in stdin.lock().lines() {

        let bin = line.unwrap().trim().to_string();

        println!("file_read: {}", bin);

        // Creates new String object, allocates memory in heap
        let mut s = String::new();
        s.push_str( &bin );

        numbers = s;     
    }




    let mut imin  : i32 = 9999;
    let mut imax  : i32 = 0;
    let mut inum  : [ i32 ; 2000 ] = [0 ; 2000];
    let mut ilen  : usize = 0;

    let xnv: Vec<&str> = numbers.split(',').collect(); 
    let xnn: usize = xnv.len();
    let mut xni: usize = 0;

    loop {
        let xnext : i32 = match xnv[xni].to_string().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        println!("entry: xni {} xnn {} xnext {}", xni, xnn, xnext);
        inum[ ilen ] = xnext;
        ilen += 1;
        if xnext > imax {
            imax = xnext;
        }
        if xnext < imin {
            imin = xnext;
        }
        
        // next number
        xni += 1;
        if xni == xnn {
            // out of numbers
            break;
        }
    }


    let mut least_fuel   : u64 = 99999999;
    let mut least_fuel_2 : u64 = 99999999;

    for i in 0..ilen {

        let move_to   : i32 = inum[ i as usize ];
        let mut sum   : u64 = 0;
        let mut sum_2 : u64 = 0;

        println!("Calc move_to [{}]", move_to);

        for j in 0..ilen {

            let mut dist : i32 = inum[ j as usize ] - move_to;
            if dist < 0 {
                dist = -dist;
            }
            sum += dist as u64;

           // arithmetic sum: S = (n * (a1 + a2)) / 2
           //let dist_2 = (dist as u64 * (1 + dist as u64)) / 2;
           let mut err_dist : i32 = inum[ j as usize ] - i as i32;
           if err_dist < 0 {
               err_dist = -err_dist;
           }
           let dist_2 = (err_dist as u64 * (1 + err_dist as u64)) / 2;

           sum_2 += dist_2;
           println!("   moving [{}] to [{}] dist {} cost {}", inum[ j as usize ], i, dist, dist_2);
        }
        println!("Sum move_to [{}] cost {}", i, sum);
        if sum < least_fuel {
            least_fuel = sum;
        }
        println!("Sum_2 move_to [{}] cost_2 {}", i, sum_2);
        if sum_2 < least_fuel_2 {
            least_fuel_2 = sum_2;
        }
    }

    println!("Least fuel:   {}", least_fuel);
    println!("Least fuel_2: {}", least_fuel_2);
}
