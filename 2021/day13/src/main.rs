
use std::io::{self, BufRead};


fn paper_print(paper : &mut [[u8 ; 1400]; 1400], maxx : usize, maxy : usize) {
    let mut mx : usize = maxx; 
    let mut my : usize = maxy; 
    if mx > 80 {
        mx = 80;
    }
    if my > 80 {
        my = 80;
    }
    println!("     MAX_X={}; MAX_Y={}", maxx, maxy);
    println!("     [x   01234567890123456789]");
    for y in 0..=my {
      if y < 10 {
        print!("PAPER[y {}]", y);
      }
      else {
        print!("PAPER[y{}]", y);
      }
      for x in 0..=mx {
          if paper[x as usize][y as usize] > 0 {
              print!("#");
          }
          else {
              print!(".");
          }
      }
      println!("");
    }
}

//-------------------------------
fn paper_fold_x(paper : &mut [[u8 ; 1400]; 1400], x_fold : usize, mut maxx : &mut usize, mut maxy : &mut usize) {
    println!("Fold_x {}", x_fold);

    let mut mx : usize = *maxx; 
    let mut my : usize = *maxy; 

   // clear vertical fold line
   for y in 0..=my {
       paper[x_fold as usize][y as usize] = 0;
   }



   // fold
   for x in 0..x_fold {
       for y in 0..=my {
           if paper[(mx - x) as usize][y as usize] > 0 {
               paper[x as usize][y as usize] = 1;
           }
       }
   }


   // calc new paper size
   let left_size  : usize = x_fold;
   let right_size : usize = mx - x_fold;
   println!("fold_x: left {} right {}", left_size, right_size);

   mx = left_size - 1;

   *maxx = mx;
   *maxy = my;
}

//-------------------------------
fn paper_fold_y(paper : &mut [[u8 ; 1400]; 1400], y_fold : usize, mut maxx : &mut usize, mut maxy : &mut usize) {
    println!("Fold_y {}", y_fold);

    let mut mx : usize = *maxx; 
    let mut my : usize = *maxy; 

   // clear horiziontal fold line
   for x in 0..=mx {
       paper[x as usize][y_fold as usize] = 0;
   }



   // fold
   for y in 0..y_fold {
       for x in 0..=mx {
           if paper[x as usize][(my - y) as usize] > 0 {
               paper[x as usize][y as usize] = 1;
           }
       }
   }

   // calc new paper size
   let top_size  : usize = y_fold;
   let bottom_size : usize = my - y_fold;
   println!("fold_y: top {} bottom {}", top_size, bottom_size);

   my = top_size - 1;

   *maxx = mx;
   *maxy = my;
}

//-------------------------------
fn paper_count_visible(paper : &mut [[u8 ; 1400]; 1400], maxx : usize, maxy : usize) -> usize {
    let mut visible : usize = 0;
    for y in 0..=maxy {
      for x in 0..=maxx {
          if paper[x as usize][y as usize] > 0 {
              visible += 1;
          }
      }
    }
    println!("Counted Visible {}", visible);
    return visible;
}

//---------------------------------
fn main() {

    let mut paper : [[u8 ; 1400]; 1400] = [[0 ; 1400]; 1400];
    let mut paper_width  : usize = 0;
    let mut paper_height : usize = 0;

    let mut read_coord_vec : Vec<String> = Vec::new();
    let mut read_fold_vec  : Vec<String> = Vec::new();
    let stdin = io::stdin();
    let mut nread : i32 = 0;
    for line in stdin.lock().lines() {
        let bin = line.unwrap().trim().to_string();
        // Creates new String object, allocates memory in heap
        let mut s = String::new();
        s.push_str( &bin );
        if s.trim() != "" {
            if s.starts_with("fold") {
                println!("read_fold: {}", bin);
                read_fold_vec.push( s );
            }
            else {
                println!("read_coord: {}", bin);
                read_coord_vec.push( s );
            }
        }
        nread += 1;
    }


    let mut maxx : u32 = 0;
    let mut maxy : u32 = 0;

    // create paper
    for dot in read_coord_vec {
        let coord: Vec<&str> = dot.split(',').collect();
        let x : u32 = match coord[0].to_string().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        let y : u32 = match coord[1].to_string().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        println!("coord x {} y {}", x, y);
        paper[x as usize][y as usize] = 1;
        if x > maxx {
            maxx = x;
        }
        if y > maxy {
            maxy = y;
        }
    }

    //---------- debug ----------------
    paper_print(&mut paper, maxx as usize, maxy as usize);
    //--------------------------


    // fold paper
    for fold in read_fold_vec {

        // is it x or y?
        if fold.contains("x=") {
            // split
            let nv: Vec<&str> = fold.split('=').collect(); 
            let crap_str : String = nv[0].to_string();
            let x_str    : String = nv[1].to_string();
            let intx : usize = match x_str.parse() {
                Ok(num) => num,
                Err(_) => break,
            };
            println!("BEFORE foldX {} my_x {} int {} sizex {} sizey {}", fold, x_str, intx, maxx, maxy);
            let mut maxx_before : usize = maxx as usize;
            let mut maxy_before : usize = maxy as usize;
            paper_fold_x(&mut paper, intx, &mut maxx_before, &mut maxy_before);
            maxx = maxx_before as u32;
            maxy = maxy_before as u32;
            println!("AFTER foldX {} my_x {} int {} sizex {} sizey {}", fold, x_str, intx, maxx, maxy);
        }
        if fold.contains("y=") {
            // split
            let nv: Vec<&str> = fold.split('=').collect(); 
            let crap_str : String = nv[0].to_string();
            let y_str    : String = nv[1].to_string();
            let inty : usize = match y_str.parse() {
                Ok(num) => num,
                Err(_) => break,
            };
            println!("BEFORE foldY {} my_y {} int {} sizex {} sizey {}", fold, y_str, inty, maxx, maxy);
            let mut maxx_before : usize = maxx as usize;
            let mut maxy_before : usize = maxy as usize;
            paper_fold_y(&mut paper, inty, &mut maxx_before, &mut maxy_before);
            maxx = maxx_before as u32;
            maxy = maxy_before as u32;
            println!("AFTER foldY {} my_y {} int {} sizex {} sizey {}", fold, y_str, inty, maxx, maxy);
        }
        println!("Interim Visible {}", paper_count_visible(&mut paper, maxx as usize, maxy as usize));
    }

    //---------- debug ----------------
    paper_print(&mut paper, maxx as usize, maxy as usize);
    //--------------------------

    println!("Paper Visible {}", paper_count_visible(&mut paper, maxx as usize, maxy as usize));
}
