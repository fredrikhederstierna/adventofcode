use std::io::{self, BufRead};

// Greedy algorithm

//-------------------------------
fn map_walk(map : &mut Box<[[u8 ; 100*5]; 100*5]>, maxrow : usize, maxcol : usize, row : usize, col : usize, cost : &mut Box<[[u16 ; 100*5]; 100*5]>, best_cost_to_here : usize) {

    //println!("Walk {},{} val {} max {} {} best {}", row, col, map[ row ][ col ], maxrow, maxcol, best_cost_to_here);

    let this_path_extra_cost : u8 = map[ row ][ col ];

    // first visist?
    if cost[ row ][ col ] == 0 {
          // first visit, set first cost anyway
          cost[ row ][ col ] = (best_cost_to_here + this_path_extra_cost as usize) as u16;
    }
    else {
        // greedy, if been here before at same or less cost, skip
        if (best_cost_to_here + this_path_extra_cost as usize) >= (cost[ row ][ col ] as usize) {
            // higher cost
            // more expensive path, dont walk further
            return;// (best_cost_to_here + this_path_extra_cost as usize) as usize;
         }
         else {
            // better path, try continue from here
            cost[ row ][ col ] = (best_cost_to_here + this_path_extra_cost as usize) as u16;
         }
    }

    let mut best_walk_cost : usize = cost[ row ][ col ] as usize;
    //let mut cost_walk      : usize;
    let mut can_walk       : bool;

    // ---- choose best direction first, greedy
    // never go back to 0,0

        
    // never go left (?) 'sweeping' left->right greedy algorithm

    
    // try left
    if col > 0 {
        can_walk = true;
        if (col - 1) == 0 {
                if row == 0 {
                        // skip
                        can_walk = false;
                }
        }
        if cost[ row ][ col - 1] > 0 {
            if cost[ row ][ col ] + map[ row ][ col - 1 ] as u16 >= cost[ row ][ col - 1] {
                  if cost[ row ][ col ] > map[ row ][ col ] as u16 + cost[ row ][ col - 1] {
                          cost[ row ][ col ] = map[ row ][ col ] as u16 + cost[ row ][ col - 1];
                          best_walk_cost = cost[ row ][ col ] as usize;
                   }
             }
            can_walk = false;
          }
            /*
        if can_walk == true {
          //print!("->L");
          /*cost_walk = */ map_walk(map, maxrow, maxcol, row, col - 1, cost, best_walk_cost);
                /*
          if best_walk_cost > cost_walk {
                  best_walk_cost = cost_walk;
           }*/
        }
            */
    }
    
    
    // try right
    if col < maxcol - 1 {
        can_walk = true;
        if cost[ row ][ col + 1] > 0 {
          if cost[ row ][ col ] + map[ row ][ col + 1 ] as u16 >= cost[ row ][ col + 1] {
                  if cost[ row ][ col ] > map[ row ][ col ] as u16 + cost[ row ][ col + 1] {
                          cost[ row ][ col ] = map[ row ][ col ] as u16 + cost[ row ][ col + 1];
                          best_walk_cost = cost[ row ][ col ] as usize;
                  }        
                  can_walk = false;
          }
        }
        if can_walk == true {
          //print!("->R");
          /* cost_walk = */ map_walk(map, maxrow, maxcol, row, col + 1, cost, best_walk_cost);
                /*
          if best_walk_cost > cost_walk {
                  best_walk_cost = cost_walk;
          }*/
        }
    }
    // try up
    if row > 0 {
        can_walk = true;
        if (row - 1) == 0 {
                if col == 0 {
                        // skip
                        can_walk = false;
                }
        }
        if cost[ row - 1 ][ col ] > 0 {
          if cost[ row ][ col ] + map[ row - 1 ][ col ] as u16 >= cost[ row - 1 ][ col ] {
                  if cost[ row ][ col ] > map[ row ][ col ] as u16 + cost[ row - 1 ][ col ] {
                          cost[ row ][ col ] = map[ row ][ col ] as u16 + cost[ row - 1 ][ col ];
                          best_walk_cost = cost[ row ][ col ] as usize;
                  }        
                  can_walk = false;
          }
        }
        if can_walk == true {
          //print!("->U");
          /* cost_walk = */ map_walk(map, maxrow, maxcol, row - 1, col, cost, best_walk_cost);
                /*
          if best_walk_cost > cost_walk {
                  best_walk_cost = cost_walk;
           }*/
        }
    }
    // try down
    if row < maxrow - 1 {
         can_walk = true;
         if cost[ row + 1 ][ col ] > 0 {
            if cost[ row ][ col ] + map[ row + 1 ][ col ] as u16 >= cost[ row + 1 ][ col ] {
                  if cost[ row ][ col ] > map[ row ][ col ] as u16 + cost[ row + 1 ][ col ] {
                          cost[ row ][ col ] = map[ row ][ col ] as u16 + cost[ row + 1 ][ col ];
                          best_walk_cost = cost[ row ][ col ] as usize;
                  }        
                  can_walk = false;
            }
         }
         if can_walk == true {
           //print!("->D");
           /* cost_walk = */ map_walk(map, maxrow, maxcol, row + 1, col, cost, best_walk_cost);
                 /*
           if best_walk_cost > cost_walk {
                 best_walk_cost = cost_walk;
            }*/
         }
    }

    /*
    if (best_walk_cost + this_path_extra_cost as usize) < (cost[ row ][ col ] as usize) {
        cost[ row ][ col ] = (best_walk_cost + this_path_extra_cost as usize) as u16;
    }
*/
    return;// cost[ row ][ col ] as usize;    
}

//-----------------------------------------
fn main() {

    let mut map_vec  : Vec<String> = Vec::new();

    let mut map  : Box<[[u8  ; 100*5]; 100*5]> = Box::new([[0 ; 100*5]; 100*5]);
    let mut cost : Box<[[u16 ; 100*5]; 100*5]> = Box::new([[0 ; 100*5]; 100*5]);

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
        println!("readPos: {}", bin);

        for (j, c) in bin.chars().enumerate() {
            // do something with character `c` and index `i`
            let six : usize = (c as usize) - ('0' as usize);
            let v : u8 = six as u8;

            // fill map for part2  5x5
            for xx in 0..5 {
                    for yy in 0..5 {
                            let mut v2 : u8 = v;
                            v2 += xx as u8;
                            if v2 > 9 {
                               v2 = v2 - 9;
                            }
                            v2 += yy as u8;
                            if v2 > 9 {
                               v2 = v2 - 9;
                            }
                            map[ i + (yy*map_height) ][ j + (xx*map_width) ] = v2;
                    }
            }
        }

    }


    // dump part1
    for r in 0..map_height {
        print!("MAP{}:", r);
        for c in 0..map_width {
            print!("{}", map[ r ][ c ]);
        }
        println!("");
    }

    // dump part2
    for r in 0..(map_height*5) {
        print!("MAP{}:", r);
        for c in 0..(map_width*5) {
            print!("{}", map[ r ][ c ]);
        }
        println!("");
    }
    

    // start pos
    map[0][0] = 0; // never go back to 0,0


    /*
    // part1
    let best_cost : usize = map_walk(&mut map, map_height, map_width, 0, 0, &mut cost, 0);
    println!("Walked {}", best_cost);
    println!("Best {}", cost[map_height - 1][map_width - 1]);
    */

    
    // part2 - small input
    //let best_cost2 : usize = map_walk(&mut map, map_height*5, map_width*5, 0, 0, &mut cost, 0);
    map_walk(&mut map, map_height*5, map_width*5, 0, 0, &mut cost, 0);
    //println!("Walked2 {}", best_cost2);
    println!("Best2 {}", cost[map_height*5 - 1][map_width*5 - 1]);
    
    
    /*
    // part2
    let best_cost2 : usize = map_walk_500(&mut map, 0, 0, &mut cost, 0);
    println!("Walked2 {}", best_cost2);
    println!("Best2 {}", cost[500 - 1][500 - 1]);
    */
}

// 538 // u+l too high
// 537 // u+l+d


// part2
// 2889 too high
