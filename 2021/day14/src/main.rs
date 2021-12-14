
use std::io::{self, BufRead};
use std::collections::HashMap;

static mut MAP: Option<HashMap<Box<u32>, Box<[u64; 30]>>> = None;

fn hashmap_init() {
    unsafe {
        MAP = Some(HashMap::new());
    }
}

fn hashmap_set(key: u32, val: [u64; 30]) {
    unsafe {
        MAP.as_mut().unwrap().insert(
            Box::new(u32::from(key)),
            Box::new(<[u64; 30]>::from(val))
        );
    }
}

fn hashmap_get(key: u32) -> [u64 ; 30] {
    unsafe {
        //return String::from(MAP.as_ref().unwrap().get(&key).unwrap().as_str());
        return <[u64; 30]>::from(**(MAP.as_ref().unwrap().get(&key).unwrap()));
    }
}

fn hashmap_contains(key: u32) -> bool {
    unsafe {
        return MAP.as_ref().unwrap().contains_key(&key);
    }
}

//-----------------------------------
fn generate_poly(ecount : &mut [u64 ; 30], char0 : char, char1: char, poly_rule: &[[char ; 3] ; 110], nrules : usize, step : usize )
{
   //println!("  -step {}: {}{}", step, poly_char0, poly_char1);

   if step == 10 {
      // count current
      let poly_char0 : char = char0;
      let poly_char1 : char = char1;
      let ix0 = poly_char0 as usize - 'A' as usize;
      let ix1 = poly_char1 as usize - 'A' as usize;
      ecount[ ix0 ] += 1;
      ecount[ ix1 ] += 1;
      return;
   }

   let poly_char0 : char = char0;
   let poly_char1 : char = char1;

       // apply rules
       for p in 0..nrules {

//println!("try {} : {} {} {}", step, poly_rule[p][0], poly_rule[p][1], poly_rule[p][2]);

           // all rules applied?
           if poly_rule[p][0] > poly_char0 {
               break;
           }
           if poly_rule[p][0] == poly_char0 {
               if poly_rule[p][1] > poly_char1 {
                   break;
               }
           }
           
           if poly_rule[p][0] == poly_char0 {
                if poly_rule[p][1] == poly_char1 {

                    let char_next0 : char = poly_rule[p][0];
                    let char_next1 : char = poly_rule[p][2];
                    
                    //println!("  recurse: old {}{} new {}{}", poly_char0, poly_char1, char_next0, char_next1); 
                    generate_poly(ecount, char_next0, char_next1, poly_rule, nrules, step + 1);
                    generate_poly(ecount, char_next1, poly_char1, poly_rule, nrules, step + 1);
                    break;
                }
            }
        }
}

//-----------------------------------
// TODO: make it return generated stuff from this point? array - dynamic programming
fn generate_poly_lookup(ecount : &mut [u64 ; 30], char0 : char, char1: char, poly_rule_lookup: &mut [[char ; 3] ; 32*32], step : usize )
{
  let poly_char0 : char = char0;
  let poly_char1 : char = char1;

  let ix0 = poly_char0 as usize - 'A' as usize;
  let ix1 = poly_char1 as usize - 'A' as usize;

  let mut lookix : usize = ix0 * 32;
  lookix += ix1 as usize;

  //println!("  -step {}: {}{}", step, poly_char0, poly_char1);

  if step == 40 {
    // count current

    ecount[ ix0 ] += 1;
    ecount[ ix1 ] += 1;

    return;
  }

  let char_next0 : char = poly_rule_lookup[lookix][0];
  let char_next1 : char = poly_rule_lookup[lookix][1];
  let char_next2 : char = poly_rule_lookup[lookix][2];

  let hkey : u32 = (ix0 << 8) as u32 + (ix1 << 16) as u32 + (step << 24) as u32;
  //hashmap_set(123456, "apple".to_string());

  let result : bool = hashmap_contains( hkey );
  if result == true {
      //println!("EXISTED");

      let mut ecount_childs : [u64 ; 30] = hashmap_get( hkey );

      for ei in 0..30 {
        ecount[ei] += ecount_childs[ei];
      }
  }
  else {
      //println!("NOt_EXISTED");
 
      let mut ecount_childs : [u64 ; 30] = [0 ; 30];
      generate_poly_lookup(&mut ecount_childs, char_next0, char_next2, poly_rule_lookup, step + 1);
      generate_poly_lookup(&mut ecount_childs, char_next2, poly_char1, poly_rule_lookup, step + 1);
      for ei in 0..30 {
        ecount[ei] += ecount_childs[ei];
      }

      hashmap_set( hkey, ecount_childs );
  }

  return;
}

//---------------------------------
fn main() {

    hashmap_init();

    let mut poly : [char ; 100*1000] = ['-' ; 100*1000];
    let mut poly_len  : usize = 0;

    let mut read_poly_rule_vec : Vec<String> = Vec::new();
    let mut start_poly : String = String::new();

    let stdin = io::stdin();

    let mut nread : i32 = 0;
    for line in stdin.lock().lines() {
        let bin = line.unwrap().trim().to_string();
        // Creates new String object, allocates memory in heap
        let mut s = String::new();
        s.push_str( &bin );
        if s.trim() != "" {
            if nread == 0 {
                println!("read_start {}", bin);
                start_poly = s;
            }
            else {
                println!("read_rule: {}", bin);
                read_poly_rule_vec.push( s );
            }
        }
        nread += 1;
    }

    let mut poly_rule : [[char ; 3] ; 110] = [['-','-','-'] ; 110];
    let mut nrules : usize = 0;

    // create rules vec
    for rule in read_poly_rule_vec {
        let src: Vec<&str> = rule.split(" -> ").collect();
        let src0_str : String = src[0].to_string();
        let src1_str : String = src[1].to_string();

        let mut src_chr : [char ; 2] = ['-', '-'];
        for (j, c) in src0_str.chars().enumerate() {
            src_chr[ j ] = c;
        }
        let mut dst_chr : char = '-';
        for (_j, c) in src1_str.chars().enumerate() {
            dst_chr = c;
        }

        println!("src {}.{} dst {}", src_chr[0], src_chr[1], dst_chr);
        poly_rule[nrules][0] = src_chr[0];
        poly_rule[nrules][1] = src_chr[1];
        poly_rule[nrules][2] = dst_chr;

        nrules += 1;
    }

    // sort rules
    for i in 0..nrules {
      for j in i+1..nrules {
          let mut swap : bool = false;
          if poly_rule[i][0] > poly_rule[j][0] {
              swap = true;
          }
          if poly_rule[i][0] == poly_rule[j][0] {
              if poly_rule[i][1] > poly_rule[j][1] {
                  swap = true;
              }
          }
          if swap == true {
             //println!("sort i {} j {}:  {} {} {} > {} {} {}", i, j, poly_rule[i][0], poly_rule[i][1], poly_rule[i][2], poly_rule[j][0], poly_rule[j][1], poly_rule[j][2]);
             let tmp0 : char = poly_rule[i][0];
             let tmp1 : char = poly_rule[i][1];
             let tmp2 : char = poly_rule[i][2];
             poly_rule[i][0] = poly_rule[j][0];
             poly_rule[i][1] = poly_rule[j][1];
             poly_rule[i][2] = poly_rule[j][2];
             poly_rule[j][0] = tmp0;
             poly_rule[j][1] = tmp1;
             poly_rule[j][2] = tmp2;
          }
      }
    }

    // print rules
    for i in 0..nrules {
        println!("sorted rule {} {} {}", poly_rule[i][0], poly_rule[i][1], poly_rule[i][2]);
    }

    // copy start poly to poly
    for (k, c) in start_poly.chars().enumerate() {
        println!("Copy start {} {}", k, c);
        poly[k] = c;
        poly_len += 1;
    }

    let mut poly_next : [char ; 100*1000] = ['-' ; 100*1000];
    let mut poly_next_len : usize;

    print!("Start: len {}: ", poly_len);
    for ii in 0..poly_len {
        print!("{}", poly[ii]);
    }
    println!("");

    let mut step : u32 = 0;
    loop {


        // apply rules
        poly_next_len = 0;
        for i in 0..(poly_len - 1) {
            //println!("Gen poly {}{}", poly[ i ], poly[ i+1 ]);

            for p in 0..nrules {
                if poly_rule[p][0] == poly[i] {
                    if poly_rule[p][1] == poly[i+1] {

                        println!("Gen poly {}{} -> {}{}{}", poly[ i ], poly[ i+1 ], poly_rule[p][0], poly_rule[p][2], poly_rule[p][1]);

                        poly_next[ poly_next_len ] = poly_rule[p][0];
                        poly_next_len += 1;
                        poly_next[ poly_next_len ] = poly_rule[p][2];
                        poly_next_len += 1;
                        //poly_next[ poly_next_len ] = poly_rule[p][1];
                        //poly_next_len += 1;

                        //break;
                    }
                }
            }
        }
        // copy last char
        poly_next[ poly_next_len ] = poly[poly_len-1];
        poly_next_len += 1;
        


        // update      
        poly = poly_next;
        poly_len = poly_next_len;



        step += 1;

        print!("Step {} len {}: ", step, poly_len);
        for ii in 0..poly_len {
            print!("{}", poly[ii]);
        }
        println!("");

        if step == 10 {
            break;
        }
    }


    // print
    print!("Final len {}: ", poly_len);
    for ii in 0..poly_len {
        print!("{}", poly[ii]);
    }
    println!("");


    // count
    let mut nelem : usize = 0;
    let mut elem : [u64 ; 30] = [0 ; 30];
    for ii in 0..poly_len {
        let ix = poly[ii] as usize - 'A' as usize;
        if ix > nelem {
            nelem = ix;
        }
        elem[ ix ] += 1;
        //println!("{} is index {}", poly[ii], ix);
    }
    elem.sort();
    let mut max_e : u64 = 0;
    let mut min_e : u64 = 6666666666666666666;
    for kk in elem {
        if kk > 0 {
            if kk > max_e {
                max_e = kk;
            }
            if kk < min_e {
                min_e = kk;
            }
        }
    }

    println!("End  min {} max {}", min_e, max_e);
    println!("Minus  {}", max_e - min_e);



    //---------------------------------------------

    // part2 restart

    // copy start poly to poly
    poly_len = 0;
    for (k, c) in start_poly.chars().enumerate() {
        println!("Re-Copy start {} {}", k, c);
        poly[k] = c;
        poly_len += 1;
    }



    let mut ecount : [u64 ; 30] = [0 ; 30];


    // apply rules
    for i in 0..(poly_len - 1) {
        let poly_char0 : char = poly[i];
        let poly_char1 : char = poly[i+1];
        println!("GEN pos={} {}{}", i, poly_char0, poly_char1);

        generate_poly(&mut ecount, poly_char0, poly_char1, &poly_rule, nrules, 0);
    }

    // count last char
    /*
    println!("LAST");
    let char_last : char = poly[ poly_len - 1 ];
    let ix_last = char_last as usize - 'A' as usize;
    ecount[ ix_last ] += 1;
    */


    ecount.sort();
    max_e = 0;
    min_e = 666666666666;
    for kk in ecount {
        if kk > 0 {
            if kk > max_e {
                max_e = kk;
            }
            if kk < min_e {
                min_e = kk;
            }
        }
    }

    println!("End2  min {} max {}", min_e, max_e);
    println!("Minus2  {}", max_e - min_e);
    println!("End2x  min {} max {}", (min_e + 1) / 2, (max_e + 1) / 2);
    println!("Minus2x  {}", (max_e + 1) / 2 - (min_e + 1) / 2);


    //--------------------------------- take 3

    // create rule lookup table


    let mut poly_rule_lookup : [[char ; 3] ; 32*32] = [['-','-','-'] ; 32*32];
    for p in 0..nrules {

           //println!("lookup {}: {} {} {}", p, poly_rule[p][0], poly_rule[p][1], poly_rule[p][2]);

           let ix0 = poly_rule[p][0] as usize - 'A' as usize;
           let ix1 = poly_rule[p][1] as usize - 'A' as usize;

           //let mut looki : usize = (poly_rule[p][0] as usize) * 32;
           //looki += poly_rule[p][1] as usize;
           let mut looki : usize = (ix0 as usize) * 32;
           looki += ix1 as usize;

           poly_rule_lookup[looki][0] = poly_rule[p][0];
           poly_rule_lookup[looki][1] = poly_rule[p][1];
           poly_rule_lookup[looki][2] = poly_rule[p][2];
    }

    let mut ecount2 : [u64 ; 30] = [0 ; 30];

    // apply rules
    for i in 0..(poly_len - 1) {
        let poly_char0 : char = poly[i];
        let poly_char1 : char = poly[i+1];
        println!("GEN2 pos={} {}{}", i, poly_char0, poly_char1);

        generate_poly_lookup(&mut ecount2, poly_char0, poly_char1, &mut poly_rule_lookup, 0);
    }

    ecount2.sort();
    max_e = 0;
    min_e = 6666666666666666666;
    for kk in ecount2 {
        if kk > 0 {
            if kk > max_e {
                max_e = kk;
            }
            if kk < min_e {
                min_e = kk;
            }
        }
    }

    println!("End3  min {} max {}", min_e, max_e);
    println!("Minus3  {}", max_e - min_e);
    println!("End3x  min {} max {}", (min_e + 1) / 2, (max_e + 1) / 2);
    println!("Minus3x  {}", (max_e + 1) / 2 - (min_e + 1) / 2);
}
