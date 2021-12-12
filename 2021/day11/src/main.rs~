use std::io::{self, BufRead};

fn map_fill_basin(basin_nbr : u8, map : [[i8 ; 100]; 100], visited : &mut [[u8 ; 100]; 100], row : usize, col : usize, maxrow : usize, maxcol : usize) -> usize {

    let mut size : usize = 1;

    visited[ row ][ col ] = basin_nbr;

    // try walk left
    if col > 0 {
        if visited[ row ][ col - 1 ] == 0 {
            if map[ row ][ col - 1] != 9 {
                size += map_fill_basin(basin_nbr, map, visited, row, col - 1, maxrow, maxcol);
            }
        }
    }
    // try walk right
    if col < maxcol - 1 {
        if visited[ row ][ col + 1 ] == 0 {
            if map[ row ][ col + 1] != 9 {
                size += map_fill_basin(basin_nbr, map, visited, row, col + 1, maxrow, maxcol);
            }
        }
    }
    // try walk up
    if row > 0 {
        if visited[ row - 1 ][ col ] == 0 {
            if map[ row - 1 ][ col ] != 9 {
                size += map_fill_basin(basin_nbr, map, visited, row - 1, col, maxrow, maxcol);
            }
        }
    }
    // try walk down
    if row < maxrow - 1 {
        if visited[ row + 1 ][ col ] == 0 {
            if map[ row + 1 ][ col ] != 9 {
                size += map_fill_basin(basin_nbr, map, visited, row + 1, col, maxrow, maxcol);
            }
        }
    }

    return size;
}

fn main() {

    let mut map_vec  : Vec<String> = Vec::new();

    let mut map : [[i8 ; 100]; 100] = [[0 ; 100]; 100];
    let mut lowp : [[bool ; 100]; 100] = [[false ; 100]; 100];

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

    let mut risk_level : i32 = 0;

    for row in 0..map_height {
        print!("ROW{}:", row);
        for col in 0..map_width {

            let mid : i8 =  map[ row ][ col ];

            print!("{}", mid);

            let mut left  : i8 = -1;
            let mut up    : i8 = -1;
            let mut right : i8 = -1;
            let mut down  : i8 = -1;

            let mut n : i8 = 0;

            let mut adj_higher : i8 = 0;
 
            // check if low point
            if col > 0 {
                left = map[ row ][ col - 1 ];
                n += 1;
            }
            if col < map_width - 1 {
                right = map[ row ][ col + 1 ];
                n += 1;
            }
            if row > 0 {
                up = map[ row - 1 ][ col ];
                n += 1;
            }
            if row < map_height - 1 {
                down = map[ row + 1 ][ col ];
                n += 1;
            }

            if left >= 0 {
                if left > mid {
                  adj_higher += 1;
                }
            }
            if right >= 0 {
                if right > mid {
                  adj_higher += 1;
                }
            }
            if up >= 0 {
                if up > mid {
                  adj_higher += 1;
                }
            }
            if down >= 0 {
                if down > mid {
                  adj_higher += 1;
                }
            }

            if adj_higher == n {
                //println!("LOW:{}", mid);
                lowp[ row ][ col ] = true;
                risk_level += 1 + mid as i32;
            }
        }
        println!("");
    }



    // part2

    let mut visited : [[u8 ; 100]; 100] = [[0 ; 100]; 100];
    let mut basin_nbr : u8 = 1;
    let mut basin_sizes : [usize; 1000] = [0 ; 1000];

    for row in 0..map_height {
        print!("R{}:", row);
        for col in 0..map_width {

            let mid : i8 = map[ row ][ col ];

            // check if low point
            if lowp[ row ] [ col ] {
                print!("X");

                // try fill
                let bsize : usize = map_fill_basin(basin_nbr, map, &mut visited, row, col, map_height, map_width);
                basin_sizes[ basin_nbr as usize ] = bsize;
                print!("<{}>", bsize);

                // next
                basin_nbr += 1;
            }
            else {
                print!("{}", mid);
            }
        }
        println!("");
    }

    // sort
    //let mut basin_sizes_sorted : [usize; 1000];
    //basin_sizes_sorted = basin_sizes.sort();
    basin_sizes.sort();
    basin_sizes.reverse();

    let mut basin_mult : usize = 1;
    for x in basin_sizes {
        print!(" {}", x);
    }

    basin_mult = basin_sizes[0] * basin_sizes[1] * basin_sizes[2];

    println!("risk_level {}", risk_level);
    println!("basins {}", basin_nbr);
    println!("basin_mult {}", basin_mult);
}
