use std::io::{self, BufRead};


fn main() {

    // part1
    let mut gamma: i32 = 0;
    let mut epsilon: i32 = 0;

    let mut nbits: usize = 0;
    let mut ones  = vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0];
    let mut zeros = vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0];

    let mut vec  : Vec<String> = Vec::new();

    // part2
    let mut oxygen: i32 = 0;
    let mut co2: i32 = 0;
    let mut vecOxy : Vec<String> = Vec::new();
    let mut vecCo2 : Vec<String> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {

        let bin = line.unwrap().trim().to_string();

        println!("read: {}", bin);

        // Creates new String object, allocates memory in heap
        let mut sox = String::new();
        sox.push_str( &bin );
        vecOxy.push( sox );
        //
        let mut sco2 = String::new();
        sco2.push_str( &bin );
        vecCo2.push( sco2 );

        vec.push(bin);
    }




    for line in vec {

        let bin = line.to_string();
        println!("read2: {}", bin);

        nbits = 0;
        //for c in bin.chars() {
        for (i, c) in bin.chars().enumerate() {
            if c == '0' {
                zeros[i] += 1;
            };
            if c == '1' {
                ones[i] += 1;
            };
            if i > nbits {
                nbits = i;
            }
        }
        nbits = nbits + 1;
    }

    for b in 1..=nbits {
        gamma <<= 1;
        epsilon <<= 1;

        if zeros[b-1] > ones[b-1] {
            println!("[{}] BIT 0 most common 0:{} 1:{}", b-1, zeros[b-1], ones[b-1]);
            epsilon += 1;
        }
        if zeros[b-1] < ones[b-1] {
            println!("[{}] BIT 1 most common 0:{} 1:{}", b-1, zeros[b-1], ones[b-1]);
            gamma += 1;
        }
        if zeros[b-1] == ones[b-1] {
            println!("ERROR");
        }
    }

    println!("gamma {} {:b} epsilon {} {:b} mult {}", gamma, gamma, epsilon, epsilon, gamma * epsilon);


    // ------------------ now test CO2:

    let mut bit_tested : usize = 0;

    loop {

        println!("oxy_bit_tested {}", bit_tested);

        // stop criterion
        let oxyLen : usize = vecOxy.len();
        println!("oxyLen {}", oxyLen);
        if oxyLen == 1 {
            break;
        }

        let mut onesOxy  = vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0];
        let mut zerosOxy = vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0];

        for i in 0..oxyLen {

            let bin = vecOxy[i].to_string();
            //println!("readOxy: {}", bin);

            for (i, c) in bin.chars().enumerate() {
                if c == '0' {
                    zerosOxy[i] += 1;
                };
                if c == '1' {
                    onesOxy[i] += 1;
                };
            }
        }

        let mut del_ones  : bool = false;
        let mut del_zeros : bool = false;
        
        // which should be removed
        if zerosOxy[bit_tested] > onesOxy[bit_tested] {
            // delete ones
            del_ones = true;
        }
        if zerosOxy[bit_tested] < onesOxy[bit_tested] {
            // delete ones
            del_zeros = true;
        }
        if zerosOxy[bit_tested] == onesOxy[bit_tested] {
            // delete ones
            del_zeros = true;
        }



        let mut maxOxyLen : usize = oxyLen;
        let mut test_pos_i : usize = 0;
        loop {

            if maxOxyLen == 1 {
                break;
            }
 
            let bin = vecOxy[ test_pos_i ].to_string();
            //println!("readOxy: {}", bin);

            let char_vec: Vec<char> = bin.chars().collect();
            let ch = char_vec[ bit_tested ];

            if del_zeros {
                if ch == '0' {
                    vecOxy.remove( test_pos_i );
                    maxOxyLen -= 1;
                    test_pos_i = 0;
                    continue;
                }
            }
            if del_ones {
                if ch == '1' {
                    vecOxy.remove( test_pos_i );
                    maxOxyLen -= 1;
                    test_pos_i = 0;
                    continue;
                }
            }

            // next value
            test_pos_i += 1;
            if test_pos_i == maxOxyLen {
                break;
            }
        }

        // next bit
        bit_tested += 1;
    }

    let oxyLen : usize = vecOxy.len();
    let mut bin_oxy_val = String::new();
    bin_oxy_val.push_str( &(vecOxy[0]) );

    //bin_oxy_val = vecOxy[0].to_string();
    println!("found Oxy: len {} val {}", oxyLen, bin_oxy_val);

    let oxy_intval = isize::from_str_radix(&bin_oxy_val, 2).unwrap();
    println!("{}", oxy_intval);

    // ------------------ now test CO2:

    let mut bit_tested : usize = 0;

    loop {

        println!("co2_bit_tested {}", bit_tested);

        // stop criterion
        let co2Len : usize = vecCo2.len();
        println!("co2Len {}", co2Len);
        if co2Len == 1 {
            break;
        }

        let mut onesCo2  = vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0];
        let mut zerosCo2 = vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0];

        for i in 0..co2Len {

            let bin = vecCo2[i].to_string();
            //println!("readCo2: {}", bin);

            for (i, c) in bin.chars().enumerate() {
                if c == '0' {
                    zerosCo2[i] += 1;
                };
                if c == '1' {
                    onesCo2[i] += 1;
                };
            }
        }

        let mut del_ones  : bool = false;
        let mut del_zeros : bool = false;
        
        // which should be removed
        if zerosCo2[bit_tested] > onesCo2[bit_tested] {
            // delete ones
            del_zeros = true;
        }
        if zerosCo2[bit_tested] < onesCo2[bit_tested] {
            // delete ones
            del_ones = true;
        }
        if zerosCo2[bit_tested] == onesCo2[bit_tested] {
            // delete ones
            del_ones = true;
        }



        let mut maxCo2Len : usize = co2Len;
        let mut test_pos_i : usize = 0;
        loop {

            if maxCo2Len == 1 {
                break;
            }
 
            let bin = vecCo2[ test_pos_i ].to_string();
            //println!("readCo2: {}", bin);

            let char_vec: Vec<char> = bin.chars().collect();
            let ch = char_vec[ bit_tested ];

            if del_zeros {
                if ch == '0' {
                    vecCo2.remove( test_pos_i );
                    maxCo2Len -= 1;
                    test_pos_i = 0;
                    continue;
                }
            }
            if del_ones {
                if ch == '1' {
                    vecCo2.remove( test_pos_i );
                    maxCo2Len -= 1;
                    test_pos_i = 0;
                    continue;
                }
            }

            // next value
            test_pos_i += 1;
            if test_pos_i == maxCo2Len {
                break;
            }
        }

        // next bit
        bit_tested += 1;
    }

    let co2Len : usize = vecCo2.len();
    let mut bin_co2_val = String::new();
    bin_co2_val.push_str( &(vecCo2[0]) );

    //bin_co2_val = vecCo2[0].to_string();
    println!("found Co2: len {} val {}", co2Len, bin_co2_val);

    let co2_intval = isize::from_str_radix(&bin_co2_val, 2).unwrap();
    println!("{}", co2_intval);

    //-------------------------

    println!("mult {}", oxy_intval * co2_intval);

}
