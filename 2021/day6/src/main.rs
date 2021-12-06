
use std::io::{self, BufRead};

fn main() {

    let mut numbers = String::new();

    let stdin = io::stdin();


    for line in stdin.lock().lines() {

        let bin = line.unwrap().trim().to_string();

        println!("read: {}", bin);

        // Creates new String object, allocates memory in heap
        let mut s = String::new();
        s.push_str( &bin );

        numbers = s;     
    }






    let mut fish_vec : [ u64 ; 9 ] = [ 0 ; 9 ];

    let xnv: Vec<&str> = numbers.split(',').collect(); 
    let xnn: usize = xnv.len();
    let mut xni: usize = 0;

    loop {
        let mut xnext : u64 = match xnv[xni].to_string().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        println!("fish_number ZERO: xni {} xnn {} xnext {}", xni, xnn, xnext);
        fish_vec[ xnext as usize ] += 1;

        // next number
        xni += 1;
        if xni == xnn {
            // out of numbers
            break;
        }
    }





    println!("Inital: {}", numbers);

    //let mut gen : i32;
    //for _gen in 1..=18 {
    for _gen in 1..=80 {
    //for _gen in 1..=256 {

        let nv: Vec<&str> = numbers.split(',').collect(); 
        let nn: usize = nv.len();
        let mut ni: usize = 0;

        let mut new_numbers = String::new();

        let mut new_fish: usize = 0;
        let mut tot_n_fish: usize = 0;

        loop {

            let mut next_draw : i32 = match nv[ni].to_string().parse() {
                Ok(num) => num,
                Err(_) => break,
            };

            //println!("draw_number: ni {} nn {} next {}", ni, nn, next_draw);

            if new_numbers.len() > 0 {
                new_numbers.push_str(",");
            }

            if next_draw == 0 {
                new_numbers.push_str( "6" );
                new_fish += 1;
            }
            else {
                next_draw -= 1;
                new_numbers.push_str( &next_draw.to_string() );
            }

            tot_n_fish += 1;

            // next number
            ni += 1;
            if ni == nn {
                // out of numbers
                break;
            }
       }
       for _x in 1..=new_fish {
            if new_numbers.len() > 0 {
                new_numbers.push_str(",");
            }
            new_numbers.push_str( "8" );
            tot_n_fish += 1;
       }
       

       numbers = new_numbers;

       //println!("After {} days: {} (total {})", _gen, numbers, tot_n_fish);
       println!("After {} days: (total {})", _gen, tot_n_fish);

   }



    for _gen in 1..=256 {
        let fish0 : u64 = fish_vec[0];
        for _ix in 1..=8 {
            fish_vec[_ix - 1] = fish_vec[_ix];
        }
        fish_vec[6] += fish0;
        fish_vec[8] = fish0;
    }
    let mut sum : u64 = 0;
    for _ix in 0..=8 {
        sum += fish_vec[_ix];
    }
    println!("sum= {}", sum);
}
