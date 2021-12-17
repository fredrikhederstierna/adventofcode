use std::io::{self, BufRead};

#[macro_use]
extern crate scan_fmt;

fn move_xy(x : &mut i32, y : &mut i32, xv : &mut i32, yv : &mut i32)
{
  *x += *xv;
  *y += *yv;

  // drag
  if *xv > 0 {
    *xv -= 1;
  }
  if *xv < 0 {
    *xv += 1;
  }
  // if zero, no change

  // gravity
  *yv -= 1;
}

fn on_target(x : i32, y : i32, tx0 : i32, ty0 : i32, tx1 : i32, ty1 : i32) -> bool
{
  if x >= tx0 {
    if x <= tx1 {
      if y >= ty0 {
        if y <= ty1 {
          return true;
        }
      }
    }
  }
  return false;
}

fn main() {

    let stdin = io::stdin();

    let mut tx0 : i32 = 0;
    let mut tx1 : i32 = 0;
    let mut ty0 : i32 = 0;
    let mut ty1 : i32 = 0;

    for line in stdin.lock().lines() {

        let bin = line.unwrap().trim().to_string();
        println!("in_read: {}", bin);

        // Creates new String object, allocates memory in heap
        let mut s = String::new();
        s.push_str( &bin );

        if let Ok((_tx0, _tx1, _ty0, _ty1)) = scan_fmt!(&bin, "target area: x={d}..{d}, y={d}..{d}", i32, i32, i32, i32) {
            tx0 = _tx0;
            tx1 = _tx1;
            ty0 = _ty0;
            ty1 = _ty1;
        }
        println!("traj x={}..{}, y={}..{}", tx0, tx1, ty0, ty1);
    }
    //-----------------------------

    let mut x : i32;
    let mut y : i32;

    let mut best_y : i32 = 0;
    let mut best_vx : i32 = 0;
    let mut best_vy : i32 = 0;

    let mut n_target_hits : u32 = 0;

    for vxtest in 1..1000 {

      for vytest in -1000..1000 {

        // start in origo
        x = 0;
        y = 0;

        // initial velocity
        let mut vx : i32 = vxtest;
        let mut vy : i32 = vytest;
        let mut maxy : i32 = 0;

        //let mut step : u32 = 0;
        let mut did_target_hit : bool = false;
        loop {
           //println!("step {}: x {} y {} vx {} vy {}", step, x, y, vx, vy);
           if on_target(x, y, tx0, ty0, tx1, ty1) {
              did_target_hit = true;
              break;
           }
           // check if passed target
           if x > tx1 {
             break;
           }
           if y < ty1 {
             break;
           }
           
           move_xy(&mut x, &mut y, &mut vx, &mut vy);
           if y > maxy {
              maxy = y;
           }
           //step += 1;
           
        } //loop

        if did_target_hit == true {
          n_target_hits += 1;
          //println!("maxy = {}", maxy);
          if maxy > best_y {
             best_y = maxy;
             best_vx = vxtest;
             best_vy = vytest;
          }
        }
     }
   }
   println!("best_y {} best_vx {} best_vy {}", best_y, best_vx, best_vy);
   println!("n_target_hits {}", n_target_hits);
}


/*

 vx0 must be so when finally vx will reach 0 when x <= tx1

 vx = vx0 - step
 if xv == 0, then must x <= tx1
 ---------
 
*/
