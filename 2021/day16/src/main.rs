use std::io::{self, BufRead};
use std::u8;

// recursive descent parsing

fn get_bits(data : [u8 ; 1000], _data_len : usize, pos : usize, mut len : usize) -> u32
{
    let mut bits : u32 = 0;
    let mut chr_pos : usize = pos / 8;
    let mut bit_pos : usize = pos % 8;
    loop {
        if len == 0 {
            break;
        }
        bits <<= 1;

        let b : u32;
        b = (data[ chr_pos ] as u32) & (1 << (7 - bit_pos));
        if b > 0 {
//            print!("1");
            bits |= 1;
        }
        else {
//            print!("0");
        }

        len -= 1;
        bit_pos += 1;
        if bit_pos == 8 {
            bit_pos = 0;
            chr_pos += 1;
        }
    }
//    println!("");
    return bits;
}

fn parse_pkt(data : [u8 ; 1000], data_len : usize, mut pos : usize, align_lit : bool, vers_sum : &mut u32, value : &mut u128) -> usize
{
    let pos_start : usize = pos;

    let v : u32;
    let t : u32;
    let i : u32;
    let mut n : u32;

    let mut subpkt_bits : u32;

    v = get_bits(data, data_len, pos, 3);
    pos += 3;
    t = get_bits(data, data_len, pos, 3);
    pos += 3;

    //println!("ver {} typ {}", v, t);

    *vers_sum += v;

    if t == 4 {
        // literal paket
        //println!("literal");
        let mut lit : u128 = 0;
        loop {
            let b : u32 = get_bits(data, data_len, pos, 5);
            pos += 5;
            //println!("LIT_DIGIT {}", b & 15);
            lit |= (b & 15) as u128;
            if b & 16 == 0 {
                // last
                break;
            }
            lit <<= 4;
        }
        println!("LITERAL {}", lit);
        if align_lit == true {
          // skip unaligned bits
          if pos % 8 > 0 {
             let skip : usize = 8 - (pos % 8);
             println!("SKIP {}", skip);
             pos += skip;
          }
        }
        *value = lit as u128;
    }
    else {
        // operator paket

        // check op
        if t == 0 {
          println!("  <<<< SUM {}", *value);
        }
        if t == 1 {
          println!("  <<<< PROD {}", *value);
        }
        if t == 2 {
          println!("  <<<< MINI {}", *value);
        }
        if t == 3 {
          println!("  <<<< MAXI {}", *value);
        }
        if t == 5 {
          println!("  <<<< GREAT {}", *value);
        }
        if t == 6 {
          println!("  <<<< LESS {}", *value);
        }
        if t == 7 {
          println!("  <<<< EQUAL {}", *value);
         }

        let mut sum   : u128 = 0;
        let mut prod  : u128 = 1; // BIG INT?
        let mut mini  : u128 = u128::MAX;//66666666666666666;
        let mut maxi  : u128 = 0;

        let mut value_left  : u128 = 0;
        let mut value_right : u128 = 0;

        i = get_bits(data, data_len, pos, 1);
        pos += 1;
        let mut pkt_cnt : usize = 0;
        if i == 0 {
          // I = 0:  -> 15 bits subpkt
          subpkt_bits = get_bits(data, data_len, pos, 15);
          pos += 15;
          println!("op subpkt len {} bits", subpkt_bits);
          loop {
              if subpkt_bits == 0 {
                  break;
              }
              let p : usize;
              let mut vers_sum_child : u32 = 0;
              let mut value_child : u128 = 0;
              p = parse_pkt(data, data_len, pos, false, &mut vers_sum_child, &mut value_child);

              // do all operations
              sum += value_child;
              prod = (prod as u128) * (value_child as u128);
              if value_child < mini {
                  mini = value_child;
              }
              if value_child > maxi {
                  maxi = value_child;
              }
              if pkt_cnt == 0 {
                  value_left = value_child;
              }
              if pkt_cnt == 1 {
                  value_right = value_child;
              }

              *vers_sum += vers_sum_child;
              subpkt_bits -= p as u32;
              pos += p;
              pkt_cnt += 1;
          }
        }
        else {
          // I = 1:  -> 11 bits N subpkt
          n = get_bits(data, data_len, pos, 11);
          pos += 11;
          println!("op N subpkt {}", n);
          loop {
              if n == 0 {
                  break;
              }
              let mut vers_sum_child : u32 = 0;
              let mut value_child : u128 = 0;
              pos += parse_pkt(data, data_len, pos, false, &mut vers_sum_child, &mut value_child);

              // do all operations
              sum += value_child;
              prod = (prod as u128) * (value_child as u128);
              if value_child < mini {
                  mini = value_child;
              }
              if value_child > maxi {
                  maxi = value_child;
              }
              if pkt_cnt == 0 {
                  value_left = value_child;
              }
              if pkt_cnt == 1 {
                  value_right = value_child;
              }

              *vers_sum += vers_sum_child;
              n -= 1;
              pkt_cnt += 1;
          }
        }


        // check op
        if t == 0 {
          // sum
          *value = sum;
          println!(" ==== SUM {}", *value);
        }
        if t == 1 {
          // prod
          *value = prod;
          println!(" ==== PROD {}", *value);
        }
        if t == 2 {
          // min
          *value = mini;
          println!(" ==== MINI {}", *value);
        }
        if t == 3 {
          // max
          *value = maxi;
          println!(" ==== MAXI {}", *value);
        }
        if t == 5 {
          // great
          if value_left > value_right {
             *value = 1;
          }
          else {
             *value = 0;
          }
          println!(" ==== GREAT {}", *value);
        }
        if t == 6 {
          // less
          if value_left < value_right {
             *value = 1;
          }
          else {
             *value = 0;
          }
          println!(" ==== LESS {}", *value);
        }
        if t == 7 {
          // eq
          if value_left == value_right {
             *value = 1;
           }
           else {
             *value = 0;
           }
          println!(" ==== EQUAL {}", *value);
         }

         
    }

    return pos - pos_start;
}

//---------------------------------------
fn parse_start(data : [u8 ; 1000], data_len : usize)
{
   let mut vers_sum : u32 = 0;
   let mut value    : u128 = 0;

   let bits_parsed : usize;
   bits_parsed = parse_pkt(data, data_len, 0, true, &mut vers_sum, &mut value);
   println!("bits parsed {}", bits_parsed);
   
   println!("VER sum {}", vers_sum);
   println!("VALUE {}",   value);
}

//-----------------------------------------
fn main() {

    let mut pkt = String::new();

    let mut data : [u8 ; 1000] = [ 0 ; 1000 ];
    let mut data_len : usize = 0;

    let stdin = io::stdin();

    for line in stdin.lock().lines() {

        let bin = line.unwrap().trim().to_string();
        println!("file_read: {}", bin);
        // Creates new String object, allocates memory in heap
        pkt.push_str( &bin );
    }
    //------------------------------

    println!("pkt {}", pkt);

    // convert hex to binary
    let mut cnt : usize = 0;
    let mut hex : [char ; 2] = ['-' ; 2];
    for (_i, c) in pkt.chars().enumerate() {
       //println!("pkt[{}]={}", i, c);
       hex[ cnt ] = c;
       cnt += 1;
       if cnt == 2 {
           let mut hex_str = String::new();
           hex_str.push_str(&hex[0].to_string());
           hex_str.push_str(&hex[1].to_string());
           print!("hex[{}:{}{}:{}]; ", data_len, hex[0], hex[1], hex_str);

           let d : u8 = u8::from_str_radix(&hex_str, 16).unwrap();
           println!("data {}", d);

           data[ data_len ] = d;
           data_len += 1;
           cnt = 0;
       }
    }
    println!("");
    //------------------------------

/*
    // ver:3  typ:3
    // VVV    TTT

    // typ=#4 is Literal
       AAAAA   1xxxx  bit[8:11]
       BBBBB   1xxxx  bit[4:7]
       CCCCC   0xxxx  bit[0:3]  0=LAST
       pad     0...

    ///---------
    // ver:3  typ:3  lenID:1
    // VVV    TTT    I

    // typ != #4 is Operator, calc on 1..N subpkt

    // I = 0:  -> 15 bits subpkt
    // I = 1:  -> 11 bits N subpkt
*/
    
    parse_start(data, data_len);
}

// Too low: you guessed 4741001363
