use std::io::{self, BufRead};

// double-linked list, backtracking

// '00'=start, '99'=end
#[derive(Copy, Clone)]
struct Cave {
    name    : [char; 2],
    small   : bool,
    big     : bool,
    start   : bool,
    end     : bool,
    visited : usize,
    nconn   : usize,
    links   : [[char; 2] ; 10]
}

fn cave_new() -> Cave {
    let cav : Cave = Cave {
        name:    [ '-', '-' ],
        small:   false,
        big:     false,
        start:   false,
        end:     false,
        visited: 0,
        nconn:   0,
        links:   [['-','-'] ; 10]
    };
    return cav;
}

//--------------------------------------
fn cave_link(caves : &mut [Cave ; 100], src_index : usize, dst_index : usize) {
    let mut found : bool = false;
    let mut src_cave : Cave = caves[ src_index ];
    let dst_cave : Cave = caves[ dst_index ];

    println!("  link {} to {}", caves[ src_index ].name[0], caves[ dst_index ].name[0]);

    for i in 0..src_cave.nconn {
        println!("  compare {} and {}", src_cave.name[0], dst_cave.links[i as usize][0]);
        if dst_cave.name[0] == src_cave.links[i as usize][0] {
            if dst_cave.name[1] == src_cave.links[i as usize][1] {
                found = true;
            }
        }
    }
    if found == false {
        println!("  copy {}{} ix {}", caves[ dst_index ].name[0],  caves[ dst_index ].name[1], caves[ src_index ].nconn);
        caves[ src_index ].links[ caves[ src_index ].nconn as usize ] = caves[ dst_index ].name;
        caves[ src_index ].nconn += 1;
    }
    println!("src_ix {} dst_ix {} found {} nconn {}", src_index, dst_index, found, src_cave.nconn);
}

//--------------------------------------
fn cave_index_of(caves : &mut [Cave ; 100], cavlen : usize, chr : [char ; 2]) -> usize {
    let mut fix : usize = 666;
    let mut ix : usize = 0;
    for _i in 1..=cavlen {
        if caves[ix].name[0] == chr[0] {
                if caves[ix].name[1] == chr[1] {
                        fix = ix;
                        break;
                }
        }                        
        ix += 1;
    }
    return fix;
}

//--------------------------------------
fn cave_backtrace(caves : &mut [Cave ; 100], cavlen : usize, cur_index : usize, start_index : usize, end_index : usize, extra_small_used : bool) -> usize {
    // did we reach end, report as valid path
    if cur_index == end_index {
            return 1;
    }

    let mut paths : usize = 0;

    if caves[ cur_index ].small {
        caves[ cur_index ].visited += 1;
    }
    
    // sum up paths from child links
    let mut nlinks : usize = 0;
    nlinks = caves[ cur_index ].nconn;
    for i in 0..nlinks {
            let mut dest_ix : usize = 0;
            dest_ix = cave_index_of( caves, cavlen, caves[ cur_index ].links[ i ] );

            // never go back to start
            if dest_ix == start_index {
                continue;
            }

            println!("trying {} paths {}", dest_ix, paths);
            if caves[ dest_ix ].visited == 0 {
                paths += cave_backtrace(caves, cavlen, dest_ix, start_index, end_index, extra_small_used);
            }
            // check extra small
            if caves[ dest_ix ].visited == 1 {
                    if caves[ dest_ix ].small {
                            if extra_small_used == false {
                                    paths += cave_backtrace(caves, cavlen, dest_ix, start_index, end_index, true);
                            }
                    }
            }
    }

    if caves[ cur_index ].small {
            caves[ cur_index ].visited -= 1;
    }

    return paths;
}

//--------------------------------------
/*
fn board_sum_undrawn(mm : &mut Board) -> i32 {
    let mut sum : i32 = 0;
    for _row in 0..=4 {
        for _col in 0..=4 {
            if mm.drawn[_row][_col] == 0 {
                sum += mm.number[_row][_col];
            }
        }
    }
    return sum;
}
*/

//------------------------------------
fn main() {

    let mut read_vec : Vec<String> = Vec::new();
    let stdin = io::stdin();
    let mut nread : i32 = 0;
    for line in stdin.lock().lines() {
        let bin = line.unwrap().trim().to_string();
        println!("read_arc: {}", bin);
        // Creates new String object, allocates memory in heap
        let mut s = String::new();
        s.push_str( &bin );
        read_vec.push( s );
        nread += 1;
    }

    //let mut caves : Vec<Cave> = Vec::new();

    let mut caves : [Cave; 100] = [ Cave { name: ['-','-'], big: false, small: false, start: false, end: false, nconn: 0, visited: 0, links: [['-','-'] ; 10] }; 100];
    let mut caves_len : usize = 0;

    let read_len : usize = read_vec.len();
    for i in 0..read_len {
        let bin = read_vec[i].to_string();
        println!("readCaveArc: {}", bin);

        // split in start dest
        let nv: Vec<&str> = bin.split('-').collect(); 
        let mut src_str : String = nv[0].to_string();
        let mut dst_str : String = nv[1].to_string();

        let mut src_is_start : bool = false;
        let mut dst_is_start : bool = false;
        let mut src_is_end   : bool = false;
        let mut dst_is_end   : bool = false;
            
        // translate
        if src_str == "start" {
            src_str = "00".to_string();
            src_is_start = true;
        }
        if src_str == "end" {
            src_str = "99".to_string();
            src_is_end = true;
        }
        if dst_str == "start" {
            dst_str = "00".to_string();
            dst_is_start = true;
        }
        if dst_str == "end" {
            dst_str = "99".to_string();
            dst_is_end = true;
        }

        println!("  arc: from {} <---> {}", src_str, dst_str);
            
        let mut src_chr : [char ; 2] = ['-', '-'];
        for (j, c) in src_str.chars().enumerate() {
            src_chr[ j ] = c;
            //println!("  write src char name {} {}", j, c);
        }
        let mut dst_chr : [char ; 2] = ['-', '-'];
        for (j, c) in dst_str.chars().enumerate() {
            dst_chr[ j ] = c;
            //println!("  write dst char name {} {}", j, c);
        }

        // check if these caves existed before, otherwise create it
        let mut src_found : bool = false;
        let mut dst_found : bool = false;
        let mut src_index : usize = 0;
        let mut dst_index : usize = 0;
        let mut src_cave : Cave = Cave { name: ['-','-'], big: false, small: false, start: false, end: false, nconn: 0, visited: 0, links: [['-','-'] ; 10] };
        let mut dst_cave : Cave = Cave { name: ['-','-'], big: false, small: false, start: false, end: false, nconn: 0, visited: 0, links: [['-','-'] ; 10] };
        let mut cur_cave : Cave = Cave { name: ['-','-'], big: false, small: false, start: false, end: false, nconn: 0, visited: 0, links: [['-','-'] ; 10] };
        //let ncaves : usize = caves.len();
        let ncaves : usize = caves_len;
        for k in 0..ncaves {
            cur_cave = caves[k];
            if cur_cave.name[0] == src_chr[0] {
                    if cur_cave.name[1] == src_chr[1] {
                        // found src
                        src_found = true;
                        src_cave = cur_cave;
                        src_index = k;
                    }
            }
            if cur_cave.name[0] == dst_chr[0] {
                    if cur_cave.name[1] == dst_chr[1] {
                        // found dst
                        dst_found = true;
                        dst_cave = cur_cave;
                        dst_index = k;
                    }
            }
        }

        println!("src {}{} dst {}{} src_found {} dst_found {}", src_chr[0], src_chr[1], dst_chr[0], dst_chr[1], src_found, dst_found);
    
        // TODO: arc start-end and end-start not valid!?
        if src_found == false {
            src_cave.name[0] = src_chr[0];
            src_cave.name[1] = src_chr[1];
            src_cave.start   = src_is_start;
            src_cave.end     = src_is_end;
            if src_chr[0] >= 'A' {
               if src_chr[0] <= 'Z' {
                   src_cave.big = true;
               }
            }
            if src_cave.big == false {
                src_cave.small = true;
            }
            src_cave.visited = 0;
            src_cave.nconn = 0;
            src_cave.links = [['-','-'] ; 10];
            //caves.push(src_cave);
            src_index = caves_len;
            caves[ caves_len ] = src_cave;
            caves_len += 1;
        }
        if dst_found == false {
            dst_cave.name[0] = dst_chr[0];
            dst_cave.name[1] = dst_chr[1];
            dst_cave.start   = dst_is_start;
            dst_cave.end     = dst_is_end;
            if dst_chr[0] >= 'A' {
               if dst_chr[0] <= 'Z' {
                   dst_cave.big = true;
               }
            }
            if dst_cave.big == false {
                dst_cave.small = true;
            }
            dst_cave.visited = 0;
            dst_cave.nconn = 0;
            dst_cave.links = [['-','-'] ; 10];
            //caves.push(dst_cave);
            dst_index = caves_len;
            caves[ caves_len ] = dst_cave;
            caves_len += 1;
        }

        // create double-links
        println!("-first link");
        cave_link(&mut caves, src_index, dst_index);
        println!("-second link");
        cave_link(&mut caves, dst_index, src_index);
        println!("-done link");
    }


    //let cavlen : usize = caves.len();
    let cavlen : usize = caves_len;            
    println!("caves_len: {}", cavlen);

    
    // what index has start and end
    let mut start_index : usize = 0;
    let mut end_index   : usize = 0;
    let mut cur_index   : usize = 0;
    
    let mut ix : usize = 0;
    for _i in 1..=cavlen {

        let cavi : &Cave = &caves[ix];

        if caves[ix].name[0] == '0' {
                if caves[ix].name[1] == '0' {
                        start_index = ix;
                }
        }
        if caves[ix].name[0] == '9' {
                if caves[ix].name[1] == '9' {
                        end_index = ix;
                }
        }
                        
        println!("{} ============ name({}{}) big({}) small({}) start({}) end({}) visited({}) nconn({})", ix, cavi.name[0], cavi.name[1], cavi.big, cavi.small, cavi.start, cavi.end, cavi.visited, cavi.nconn);
        for p in 0..cavi.nconn {
            println!("    link {}{}", cavi.links[p as usize][0], cavi.links[p as usize][1]);
        }
        //cave_print(cavi);
        //println!("++++++++++++");

        ix += 1;
    }

    cur_index = start_index;

    println!("cur_ix {} start_ix {} end_ix {}", cur_index, start_index, end_index);

    start_index = cave_index_of(&mut caves, cavlen, ['0', '0']);
    end_index   = cave_index_of(&mut caves, cavlen, ['9', '9']);

    println!("cur_ix {} start_ix {} end_ix {}", cur_index, start_index, end_index);

    println!("=== backtrace start");
    let mut paths : usize = 0;
    paths = cave_backtrace(&mut caves, cavlen, cur_index, start_index, end_index, false);
    println!("=== backtrace paths {}", paths);
}
