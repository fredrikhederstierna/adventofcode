
use std::io::{self, BufRead};

#[macro_use]
extern crate scan_fmt;

//#[macro_use]
//extern crate text_io;

struct Board {
  number : [[i32; 1000]; 1000],
}

fn board_new() -> Board {
    let brd : Board = Board {
        number: [ [0; 1000] ; 1000 ],
    };
    return brd;
}

fn main() {

    let mut matrix_vec  : Vec<String> = Vec::new();

    let stdin = io::stdin();

    let mut nread : i32 = 0;

    for line in stdin.lock().lines() {

        let bin = line.unwrap().trim().to_string();

        println!("in_read[{}]: {}", nread, bin);

        // Creates new String object, allocates memory in heap
        let mut s = String::new();
        s.push_str( &bin );

        matrix_vec.push( s );
     
        nread += 1;
    }


    let mut my_board : Board = board_new();


    let mlen : usize = matrix_vec.len();
    let mut x1 : i32 = 0;
    let mut x2 : i32 = 0;
    let mut y1 : i32 = 0;
    let mut y2 : i32 = 0;
    for i in 0..mlen {
        let bin = matrix_vec[i].to_string();
        println!("readMat: {}", bin);

        if let Ok((_x1, _y1, _x2, _y2)) = scan_fmt!(&bin, "{d},{d} -> {d},{d}", i32, i32, i32, i32) {
            x1 = _x1;
            y1 = _y1;
            x2 = _x2;
            y2 = _y2;
        }
        println!("{},{} -> {},{}", x1, y1, x2, y2);

        // try draw horizontal 
        if x1 > x2 {
            // swap
            let tempx : i32 = x1;
            x1 = x2;
            x2 = tempx;

            let tempy : i32 = y1;
            y1 = y2;
            y2 = tempy;
        }

        // check yslope
        let mut yslope : i32 = 0;
        if y1 > y2 {
            yslope = -1;
        }
        if y2 > y1 {
            yslope = 1;
        }

        let mut y : i32 = y1;

        if x1 == x2 {
          // vertical line
          if y1 > y2 {
              // swap
              let tempy : i32 = y1;
              y1 = y2;
              y2 = tempy;
          }        
          for y in y1..=y2 {
              my_board.number[y as usize][x1 as usize] += 1;
          }
        }
        if x1 != x2 {
          for x in x1..=x2 {
              my_board.number[y as usize][x as usize] += 1;
              y += yslope;
          }
        }
    }



    let mut nn : i32 = 0;
    for row in 0..10 {
        for col in 0..10 {
            print!("{}", my_board.number[row][col]);
        }
        println!("");
    }



    let mut n : i32 = 0;
    for row in 0..1000 {
        for col in 0..1000 {
            if my_board.number[row][col] > 1 {
                n += 1;
                //println!("{} {},", my_board.number[row][col], n);
            }
        }
    }
    println!("overlap {}", n);
}
