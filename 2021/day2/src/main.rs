use std::io::{self, BufRead};

fn main() {

    // part1
    let mut depth: i32 = 0;
    let mut forward: i32 = 0;

    // part2
    let mut depth2: i32 = 0;
    let mut forward2: i32 = 0;
    let mut aim2: i32 = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {

        let dir = line.unwrap().trim().to_string();
        let v: Vec<&str> = dir.split_terminator(' ').collect();
        let direct = v[0].to_string();
        let readval: i32 = match v[1].parse() {
            Ok(num) => num,
            Err(_) => -666, //break,
        };
        println!("read: {},{}", direct, readval);

        // part1
        match &direct[..] {
            "up" => { println!("Go UP"); depth = depth - readval },
            "down" => { println!("Go DOWN"); depth = depth + readval },
            "forward" => { println!("Go FORWARD"); forward = forward + readval },
            _ => println!("Error"),
        }
        println!("depth {} forward {}", depth, forward);

        // part2
        match &direct[..] {
            "up" => { println!("Go2 UP"); aim2 = aim2 - readval },
            "down" => { println!("Go2 DOWN"); aim2 = aim2 + readval },
            "forward" => { println!("Go2 FORWARD"); forward2 = forward2 + readval; depth2 = depth2 + (readval * aim2); },
            _ => println!("Error"),
        }
        println!("depth {} forward {}", depth, forward);
        println!("depth2 {} forward2 {} aim2 {}", depth2, forward2, aim2);

    }
    println!("depth {} forward {} mult {}", depth, forward, depth * forward);
    println!("depth2 {} forward2 {} aim2 {} mult2 {}", depth2, forward2, aim2, depth2 * forward2);
}
