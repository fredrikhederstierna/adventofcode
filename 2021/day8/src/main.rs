use std::io::{self, BufRead};

fn main() {

    let mut sig_vec  : Vec<String> = Vec::new();

    let stdin = io::stdin();

    for line in stdin.lock().lines() {

        let bin = line.unwrap().trim().to_string();

        println!("file_read: {}", bin);

        // Creates new String object, allocates memory in heap
        let mut s = String::new();
        s.push_str( &bin );

        // signals
        sig_vec.push( s );
    }
    //------------------------------

    // part1
    let mut unique : i32 = 0;

    // part2
    let mut total_sum : u64 = 0;

    let sig_len : usize = sig_vec.len();
    for i in 0..sig_len {

        let mut insig_vec : [[bool ; 7]; 10] = [[false ; 7]; 10];
        let mut insig_nbr : usize = 0;

        let mut trans_vec : [char ; 10] = ['-'; 10];

        let mut outval_read : i32 = 0;
        let mut outvalues : [i32 ; 4] = [0; 4];

        let mut delim_read : bool = false;

        let bin = sig_vec[i].to_string();
        println!("readSig: {}  delim --> {}", bin, delim_read);

        let mut iter = bin.split_whitespace();
        loop {
            let s1 : String = iter.next().unwrap().to_string();
            println!("readIter: {}  delim --> {}", s1, delim_read);

            if delim_read {

                // ---- delimiter read ---- set outval


                // input signals
                let mut cv : [char ; 7] = ['0'; 7];
                let mut cv_ones : i32 = 0;
                for (i, c) in s1.chars().enumerate() {

                    // do something with character `c` and index `i`
                    let six : usize = (c as usize) - ('a' as usize);

                    cv[ six ] = '1';
                    cv_ones += 1;
                    
                    // do something with character `c` and index `i`
                    println!("out_char at {} is {} ix {}", i, c, six);
                }
                print!("OUT:");
                for kx in cv {
                    print!("{}", kx);
                }
                println!("");


                // ok, translate!  map cv ==
                let mut mapped_value : i32 = 0;


                for aix in 0..=9 {
                    let mut ones : i32 = 0;
                    let mut common_ones : i32 = 0;
                    for six in 0..=6 {
                        if insig_vec[ aix ][ six ] == true {
                            ones += 1;
                            if cv[ six ] == '1' {
                                 common_ones += 1;
                            }
                        }
                     }
                     if ones == cv_ones {
                         if common_ones == cv_ones {
                            // found
                            mapped_value = (trans_vec[aix] as i32) - ('0' as i32);
                            break;
                         }
                      }
                }






                outvalues[ outval_read as usize ] = mapped_value;



                // part1
                let pat_len : usize = s1.chars().count();
                //println!("patlen {}", pat_len);
                if pat_len == 2 {
                    unique += 1;
                }
                if pat_len == 3 {
                    unique += 1;
                }
                if pat_len == 4 {
                    unique += 1;
                }
                if pat_len == 7 {
                    unique += 1;
                }
                // part1 end.




                outval_read += 1;
                if outval_read == 4 {
                    let mut kk : usize = 0;
                    let mut result : i32 = 0;
                    for o in outvalues {
                        println!("OUT {} VAL {}", kk, o);
                        result *= 10;
                        result += o;
                        kk += 1;
                    }
                    println!("multi={}", result);
                    total_sum += result as u64;
                    break;
                }
            }
            else {


                // ---- still check for delimiter
                let ch = s1.chars().next().unwrap();
                if ch == '|' {

                   // all insignals read, now try map
                   println!("DELIM READ: Mapping...");


                   let mut index_num_1 : usize = 0;
                   let mut index_num_2 : usize = 0;
                   let mut index_num_3 : usize = 0;
                   let mut index_num_4 : usize = 0;
                   let mut index_num_5 : usize = 0;
                   let mut index_num_6 : usize = 0;
                   let mut index_num_7 : usize = 0;
                   let mut index_num_8 : usize = 0;
                   let mut index_num_9 : usize = 0;
                   let mut index_num_0 : usize = 0;

                   // find '1'
                   for aix in 0..=9 {
                      let mut ones : i32 = 0;
                      for six in 0..=6 {
                          if insig_vec[ aix ][ six ] == true {
                              ones += 1;
                          }
                      }
                      if ones == 2 {
                          // found '1'
                          trans_vec[aix] = '1';
                          index_num_1 = aix;
                          break;
                      }
                   }
                   // find '7'
                   for aix in 0..=9 {
                      let mut ones : i32 = 0;
                      for six in 0..=6 {
                          if insig_vec[ aix ][ six ] == true {
                              ones += 1;
                          }
                      }
                      if ones == 3 {
                          // found '7'
                          trans_vec[aix] = '7';
                          index_num_7 = aix;
                          break;
                      }
                   }
                   // find '4'
                   for aix in 0..=9 {
                      let mut ones : i32 = 0;
                      for six in 0..=6 {
                          if insig_vec[ aix ][ six ] == true {
                              ones += 1;
                          }
                      }
                      if ones == 4 {
                          // found '4'
                          trans_vec[aix] = '4';
                          index_num_4 = aix;
                          break;
                      }
                   }
                   // find '8'
                   for aix in 0..=9 {
                      let mut ones : i32 = 0;
                      for six in 0..=6 {
                          if insig_vec[ aix ][ six ] == true {
                              ones += 1;
                          }
                      }
                      if ones == 7 {
                          // found '8'
                          trans_vec[aix] = '8';
                          index_num_8 = aix;
                          break;
                      }
                   }
                   //------

                   // find '6 - it is the digit that got 6 letters, and 1 common with '1'
                   for aix in 0..=9 {
                       if trans_vec[aix] != '-' {
                           continue;
                       }
                       let mut ones : i32 = 0;
                       let mut common_ones_1 : i32 = 0;
                       for six in 0..=6 {
                           if insig_vec[ aix ][ six ] == true {
                               ones += 1;
                               if insig_vec[ index_num_1 ][ six ] == true {
                                   common_ones_1 += 1;
                               }
                           }
                       }
                       if ones == 6 {
                           if common_ones_1 == 1 {
                               // found '6'
                               trans_vec[aix] = '6';
                               index_num_6 = aix;
                               break;
                           }
                       }
                   }


                   // find '5 - it is the digit that got 5 letters, and 5 common with '6'
                   for aix in 0..=9 {
                       if trans_vec[aix] != '-' {
                           continue;
                       }
                       let mut ones : i32 = 0;
                       let mut common_ones_6 : i32 = 0;
                       for six in 0..=6 {
                           if insig_vec[ aix ][ six ] == true {
                               ones += 1;
                               if insig_vec[ index_num_6 ][ six ] == true {
                                   common_ones_6 += 1;
                               }
                           }
                       }
                       if ones == 5 {
                           if common_ones_6 == 5 {
                               // found '5'
                               trans_vec[aix] = '5';
                               index_num_5 = aix;
                               break;
                           }
                       }
                   }


                   // find '3' - it is the digit that got 5 letters, and 2 common with '1'
                   for aix in 0..=9 {
                       if trans_vec[aix] != '-' {
                           continue;
                       }
                       let mut ones : i32 = 0;
                       let mut common_ones_1 : i32 = 0;
                       for six in 0..=6 {
                           if insig_vec[ aix ][ six ] == true {
                               ones += 1;
                               if insig_vec[ index_num_1 ][ six ] == true {
                                   common_ones_1 += 1;
                               }
                           }
                       }
                       if ones == 5 {
                           if common_ones_1 == 2 {
                               // found '3'
                               trans_vec[aix] = '3';
                               index_num_3 = aix;
                               break;
                           }
                       }
                   }


                   // last unalloced with 5 letters is '2'
                   for aix in 0..=9 {
                       if trans_vec[aix] != '-' {
                           continue;
                       }
                       let mut ones : i32 = 0;
                       for six in 0..=6 {
                           if insig_vec[ aix ][ six ] == true {
                               ones += 1;
                           }
                       }
                       if ones == 5 {
                           // found '2'
                           trans_vec[aix] = '2';
                           index_num_2 = aix;
                           break;
                       }
                   }


                   // find '9' - it is the digit that got 6 letters, and 5 common with '3'
                   for aix in 0..=9 {
                       if trans_vec[aix] != '-' {
                           continue;
                       }
                       let mut ones : i32 = 0;
                       let mut common_ones_3 : i32 = 0;
                       for six in 0..=6 {
                           if insig_vec[ aix ][ six ] == true {
                               ones += 1;
                               if insig_vec[ index_num_3 ][ six ] == true {
                                   common_ones_3 += 1;
                               }
                           }
                       }
                       if ones == 6 {
                           if common_ones_3 == 5 {
                               // found '9'
                               trans_vec[aix] = '9';
                               index_num_9 = aix;
                               break;
                           }
                       }
                   }


                   // last unalloced is '0'
                   for aix in 0..=9 {
                       if trans_vec[aix] != '-' {
                           continue;
                       }
                       // found '0'
                       trans_vec[aix] = '0';
                       index_num_0 = aix;
                       break;
                   }

                   //-------
                   


                   // --- DUMP MAP RESULTS
                   println!("         abcdefg");
                   for aix in 0..=9 {
                       print!("[DIGIT{}]:", aix);
                       for six in 0..=6 {
                           if insig_vec[ aix ][ six ] == true { print!("1"); }
                           else { print!("0"); }
                       }
                       println!(" [{}]", trans_vec[aix]);
                   }
                    


                   delim_read = true;
                   continue;
                }



                // -------- input signals

                for (i, c) in s1.chars().enumerate() {

                    // do something with character `c` and index `i`
                    let seg_ix : usize = (c as usize) - ('a' as usize);

                    println!("insig[{}] char at {} is {} - ix {}", insig_nbr, i, c, seg_ix);

                    insig_vec[ insig_nbr ][ seg_ix ] = true;

                } // all letters in insig


                // next insignal
                insig_nbr += 1;
            }
        }
    }

    println!("unique = {}", unique);
    println!("total_sum = {}", total_sum);
}
