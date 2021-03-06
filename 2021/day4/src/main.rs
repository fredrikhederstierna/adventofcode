use std::io::{self, BufRead};

struct Board {
  number : [[i32; 5]; 5],
  drawn  : [[i32; 5]; 5]
}

fn board_new() -> Board {
    let brd : Board = Board {
        number: [ [0,0,0,0,0],[0,0,0,0,0],[0,0,0,0,0],[0,0,0,0,0],[0,0,0,0,0] ],
        drawn:  [ [0,0,0,0,0],[0,0,0,0,0],[0,0,0,0,0],[0,0,0,0,0],[0,0,0,0,0] ]
    };
    return brd;
}

fn board_print(mm : &Board) {
    println!("{}({}) {}({}) {}({}) {}({}) {}({})", mm.number[0][0], mm.drawn[0][0], mm.number[0][1], mm.drawn[0][1], mm.number[0][2], mm.drawn[0][2], mm.number[0][3], mm.drawn[0][3], mm.number[0][4], mm.drawn[0][4]);
    println!("{}({}) {}({}) {}({}) {}({}) {}({})", mm.number[1][0], mm.drawn[1][0], mm.number[1][1], mm.drawn[1][1], mm.number[1][2], mm.drawn[1][2], mm.number[1][3], mm.drawn[1][3], mm.number[1][4], mm.drawn[1][4]);
    println!("{}({}) {}({}) {}({}) {}({}) {}({})", mm.number[2][0], mm.drawn[2][0], mm.number[2][1], mm.drawn[2][1], mm.number[2][2], mm.drawn[2][2], mm.number[2][3], mm.drawn[2][3], mm.number[2][4], mm.drawn[2][4]);
    println!("{}({}) {}({}) {}({}) {}({}) {}({})", mm.number[3][0], mm.drawn[3][0], mm.number[3][1], mm.drawn[3][1], mm.number[3][2], mm.drawn[3][2], mm.number[3][3], mm.drawn[3][3], mm.number[3][4], mm.drawn[3][4]);
    println!("{}({}) {}({}) {}({}) {}({}) {}({})", mm.number[4][0], mm.drawn[4][0], mm.number[4][1], mm.drawn[4][1], mm.number[4][2], mm.drawn[4][2], mm.number[4][3], mm.drawn[4][3], mm.number[4][4], mm.drawn[4][4]);
}

fn board_draw(mm : &mut Board, number : i32) {
    for _row in 0..=4 {
        for _col in 0..=4 {
            if mm.number[_row][_col] == number {
                mm.drawn[_row][_col] = 1;
                return;
            }
        }
    }
}

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

fn board_is_bingo(mm : &Board) -> bool {
    let mut bingo : bool = false;
    let mut row : i32;
    let mut col : i32;
    for _i in 0..4 {
        row = 0;
        col = 0;
        if mm.drawn[_i][0] == 1 {
            row += 1;
        }
        if mm.drawn[_i][1] == 1 {
            row += 1;
        }
        if mm.drawn[_i][2] == 1 {
            row += 1;
        }
        if mm.drawn[_i][3] == 1 {
            row += 1;
        }
        if mm.drawn[_i][4] == 1 {
            row += 1;
        }

        if row == 5 {
            bingo = true;
            break;
        }

        if mm.drawn[0][_i] == 1 {
            col += 1;
        }
        if mm.drawn[1][_i] == 1 {
            col += 1;
        }
        if mm.drawn[2][_i] == 1 {
            col += 1;
        }
        if mm.drawn[3][_i] == 1 {
            col += 1;
        }
        if mm.drawn[4][_i] == 1 {
            col += 1;
        }

        if col == 5 {
            bingo = true;
            break;
        }
    }
    return bingo;
}

fn main() {


    let mut numbers = String::new();
    let mut matrix_vec  : Vec<String> = Vec::new();

    let stdin = io::stdin();

    let mut nread : i32 = 0;

    for line in stdin.lock().lines() {

        let bin = line.unwrap().trim().to_string();

        println!("read: {}", bin);

        // Creates new String object, allocates memory in heap
        let mut s = String::new();
        s.push_str( &bin );

        if nread == 0 {
            numbers = s;
        }
        else {
            if s.trim() != "" {
                matrix_vec.push( s );
            }
        }
     
        nread += 1;
    }

    let mut matrix : Vec<Board> = Vec::new();

    let mut mrow : usize = 0;
    let mut my_board : Board = board_new();

    let mlen : usize = matrix_vec.len();
    for i in 0..mlen {
        let bin = matrix_vec[i].to_string();
        //println!("readMat: {}", bin);

        let mut iter = bin.split_whitespace();

        let c1 : i32 = match iter.next().unwrap().to_string().parse()  {
            Ok(num) => num,
            Err(_) => break,
        };
        let c2 : i32 = match iter.next().unwrap().to_string().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        let c3 : i32 = match iter.next().unwrap().to_string().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        let c4 : i32 = match iter.next().unwrap().to_string().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        let c5 : i32 = match iter.next().unwrap().to_string().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        println!("next: {} {} {} {} {}", c1,c2,c3,c4,c5);
        
        my_board.number[mrow][0] = c1;
        my_board.drawn[mrow][0] = 0;
        my_board.number[mrow][1] = c2;
        my_board.drawn[mrow][1] = 0;
        my_board.number[mrow][2] = c3;
        my_board.drawn[mrow][2] = 0;
        my_board.number[mrow][3] = c4;
        my_board.drawn[mrow][3] = 0;
        my_board.number[mrow][4] = c5;
        my_board.drawn[mrow][4] = 0;

        mrow += 1;
        if mrow == 5 {
            matrix.push(my_board);
            my_board = board_new();
            mrow = 0;        
        }
    }


    let matlen : usize = matrix.len();
    println!("matlen: {}", matlen);

    let mut ix : usize = 0;
    for _i in 1..=matlen {

        let mm : &Board = &matrix[ix];

        println!("============");
        board_print(mm);
        println!("++++++++++++");

        ix += 1;
    }


    println!("draw_order: {}", numbers);
    let nv: Vec<&str> = numbers.split(',').collect(); 
    let nn: usize = nv.len();
    let mut ni: usize = 0;
 
    let mut bingo_count : i32 = 0;

    loop {

        let next_draw : i32 = match nv[ni].to_string().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        println!("draw_number: ni {} nn {} next {}", ni, nn, next_draw);

  
        let mut ix : usize = 0;
        for _i in 1..=matlen {

            let mm : &mut Board = &mut matrix[ix];

            // skip if already bingo
            let mut board_bingo : bool;
            board_bingo = board_is_bingo(mm);
            if board_bingo {
                ix += 1;
                continue;
            }

            // mark number
            board_draw(mm, next_draw);

            // check if new bingo
            board_bingo = board_is_bingo(mm);
            if board_bingo {
                 bingo_count += 1;
                 println!("BINGO {} board {}", bingo_count, ix);
                 board_print(mm);

                 let sum : i32 = board_sum_undrawn(mm);
                 println!("sum unmarked {} last draw {} mult {}", sum, next_draw, next_draw * sum);
            }
 
            ix += 1;
        }




        // next number
        ni += 1;
        if ni == nn {
            // out of numbers
            break;
        }

    }



}
