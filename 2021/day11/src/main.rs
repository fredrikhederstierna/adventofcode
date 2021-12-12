use std::io::{self, BufRead};

fn map_flash(map : &mut [[i8 ; 10]; 10], flashed : &mut [[bool ; 10]; 10], row : usize, col : usize, maxrow : usize, maxcol : usize) -> usize {

    let mut nflash : usize = 0;

    if flashed[ row ][ col ] {
       return 0;
    }

    //println!("Flash {},{} val {}", row, col, map[ row ][ col ]);
    map[ row ][ col ] = 0;
    flashed[ row ][ col ] = true;
    nflash += 1;

    // try left
    if col > 0 {
        if flashed[ row ][ col - 1] == false {
            map[ row ][ col - 1] += 1;
            if map[ row ][ col - 1] > 9 {
                nflash += map_flash(map, flashed, row, col - 1, maxrow, maxcol);
            }
        }
    }
    // try right
    if col < maxcol - 1 {
        if flashed[ row ][ col + 1] == false {
            map[ row ][ col + 1] += 1;
            if map[ row ][ col + 1] > 9 {
                nflash += map_flash(map, flashed, row, col + 1, maxrow, maxcol);
            }
        }
    }
    // try up
    if row > 0 {
        if flashed[ row - 1 ][ col ] == false {
            map[ row - 1 ][ col ] += 1;
            if map[ row - 1 ][ col ] > 9 {
                nflash += map_flash(map, flashed, row - 1, col, maxrow, maxcol);
            }
        }
    }
    // try down
    if row < maxrow - 1 {
        if flashed[ row + 1 ][ col ] == false {
            map[ row + 1 ][ col ] += 1;
            if map[ row + 1 ][ col ] > 9 {
                nflash += map_flash(map, flashed, row + 1, col, maxrow, maxcol);
            }
        }
    }


    // try left up
    if col > 0 {
        if row > 0 {
            if flashed[ row - 1 ][ col - 1] == false {
                map[ row - 1 ][ col - 1] += 1;
                if map[ row - 1 ][ col - 1] > 9 {
                    nflash += map_flash(map, flashed, row - 1, col - 1, maxrow, maxcol);
                }
            }
        }
    }
    // try right up
    if col < maxcol - 1 {
        if row > 0 {
            if flashed[ row - 1 ][ col + 1] == false {
                map[ row - 1 ][ col + 1] += 1;
                if map[ row - 1 ][ col + 1] > 9 {
                    nflash += map_flash(map, flashed, row - 1, col + 1, maxrow, maxcol);
                }
            }
        }
    }
    // try left down
    if col > 0 {
        if row < maxrow - 1 {
            if flashed[ row + 1 ][ col - 1] == false {
                map[ row + 1 ][ col - 1] += 1;
                if map[ row + 1 ][ col - 1] > 9 {
                    nflash += map_flash(map, flashed, row + 1, col - 1, maxrow, maxcol);
                }
            }
        }
    }
    // try right down
    if col < maxcol - 1 {
        if row < maxrow - 1 {
            if flashed[ row + 1 ][ col + 1] == false {
                map[ row + 1 ][ col + 1] += 1;
                if map[ row + 1 ][ col + 1] > 9 {
                    nflash += map_flash(map, flashed, row + 1, col + 1, maxrow, maxcol);
                }
            }
        }
    }


    return nflash;
}

//-----------------------------------------
fn main() {

    let mut map_vec  : Vec<String> = Vec::new();

    let mut map : [[i8 ; 10]; 10] = [[0 ; 10]; 10];

    let mut map_width  : usize = 0;
    let mut map_height : usize = 0;

    let stdin = io::stdin();

    for line in stdin.lock().lines() {

        let bin = line.unwrap().trim().to_string();

        println!("file_read: {}", bin);

        // Creates new String object, allocates memory in heap
        let mut s = String::new();
        s.push_str( &bin );

        map_height += 1;
        map_width = s.len();

        map_vec.push( s );
    }
    //------------------------------


    println!("map size: width {} height {}", map_width, map_height);

    let map_len : usize = map_vec.len();
    for i in 0..map_len {

        let bin = map_vec[i].to_string();
        println!("readSig: {}", bin);

        for (j, c) in bin.chars().enumerate() {
            // do something with character `c` and index `i`
            let six : usize = (c as usize) - ('0' as usize);
            map[ i ][ j ] = six as i8;
        }

    }


    let mut gen : u32 = 0;
    let mut tot_flashed : u64 = 0;
    let mut all_flashed : bool = false;

    loop {
        println!("Generation {}", gen);

        // dump
        for row in 0..map_height {
            print!("MAP{}:", row);
            for col in 0..map_width {
                print!("{}", map[ row ][ col ]);
            }
            println!("");
        }

        if gen == 1000 {
            break;
        }
        if all_flashed {
            break;
        }


        // increase all
        for row in 0..map_height {
            for col in 0..map_width {
               let mut mid : i8 = map[ row ][ col ];
               // just increase
               mid += 1;
               map[ row ][ col ] = mid;
            }
        }


        let mut flashed : [[bool ; 10]; 10] = [[false ; 10]; 10];
        let mut nflash : usize = 0;

        // check flash
        for row in 0..map_height {
            for col in 0..map_width {
               let mut mid : i8 = map[ row ][ col ];
               // check flash
               if mid > 9 {
                   nflash += map_flash(&mut map, &mut flashed, row, col, map_height, map_width);
               }
            }
        }
        tot_flashed += nflash as u64;
        println!("Flashed {} Total {}", nflash, tot_flashed);

        // part 2
        if nflash == 100 {
            all_flashed = true;
        }


        gen += 1;
        
    } // loop

    println!("Total flashed {}", tot_flashed);
}
