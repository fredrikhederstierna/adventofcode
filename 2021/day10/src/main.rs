use std::io::{self, BufRead};

use std::collections::LinkedList;

fn main() {

    let mut sx_vec  : Vec<String> = Vec::new();

    let mut auto_vec  : Vec<usize> = Vec::new();

    let mut n : usize = 0;

    let stdin = io::stdin();

    for line in stdin.lock().lines() {

        let bin = line.unwrap().trim().to_string();

        println!("file_read: {}", bin);

        // Creates new String object, allocates memory in heap
        let mut s = String::new();
        s.push_str( &bin );

        n += 1;

        sx_vec.push( s );
    }
    println!("n size: {}", n);
    //------------------------------


    let mut para_list = LinkedList::new();
    let mut clam_list = LinkedList::new();
    let mut curl_list = LinkedList::new();
    let mut less_list = LinkedList::new();


    let mut open_para : usize = 0;
    let mut open_clam : usize = 0;
    let mut open_curl : usize = 0;
    let mut open_less : usize = 0;

    let mut open_para_vec : [usize ; 200] = [ 0; 200];
    let mut open_clam_vec : [usize ; 200] = [ 0; 200];
    let mut open_curl_vec : [usize ; 200] = [ 0; 200];
    let mut open_less_vec : [usize ; 200] = [ 0; 200];

    let mut nsyntax : usize = 0;
    let mut last_char : char = '-';
    let mut score : usize = 0;

    let sx_len : usize = sx_vec.len();
    for i in 0..sx_len {

        let bin = sx_vec[i].to_string();
        println!("read syntax: {}", bin);

        let mut syntax_error : bool = false;

        let mut pos : usize = 0;

        for (j, c) in bin.chars().enumerate() {
            // do something with character `c` and index `i`
            print!("char[{}] {}", j, c);

            last_char = c;
 
            // open
            if c == '(' {
                open_para += 1;
                para_list.push_back( open_clam );
                para_list.push_back( open_curl );
                para_list.push_back( open_less );
            }
            if c == '[' {
                open_clam += 1;
                clam_list.push_back( open_para );
                clam_list.push_back( open_curl );
                clam_list.push_back( open_less );
            }
            if c == '{' {
                open_curl += 1;
                curl_list.push_back( open_para );
                curl_list.push_back( open_clam );
                curl_list.push_back( open_less );
            }
            if c == '<' {
                open_less += 1;
                less_list.push_back( open_para );
                less_list.push_back( open_clam );
                less_list.push_back( open_curl );
            }

            // close
            if c == ')' {
                if open_para == 0 {
                    syntax_error = true;
                    break;
                }
                let start_less : usize = para_list.pop_back().unwrap();
                let start_curl : usize = para_list.pop_back().unwrap();
                let start_clam : usize = para_list.pop_back().unwrap();
                println!("");
                println!("start clam {} curl {} less {}", start_clam, start_curl, start_less);
                if start_clam != open_clam {
                    syntax_error = true;
                    break;
                }
                if start_curl != open_curl {
                    syntax_error = true;
                    break;
                }
                if start_less != open_less {
                    syntax_error = true;
                    break;
                }
                open_para -= 1;
            }
            if c == ']' {
                if open_clam == 0 {
                    syntax_error = true;
                    break;
                }
                let start_less : usize = clam_list.pop_back().unwrap();
                let start_curl : usize = clam_list.pop_back().unwrap();
                let start_para : usize = clam_list.pop_back().unwrap();
                println!("");
                println!("start para {} curl {} less {}", start_para, start_curl, start_less);
                if start_para != open_para {
                    syntax_error = true;
                    break;
                }
                if start_curl != open_curl {
                    syntax_error = true;
                    break;
                }
                if start_less != open_less {
                    syntax_error = true;
                    break;
                }
                open_clam -= 1;
            }
            if c == '}' {
                if open_curl == 0 {
                    syntax_error = true;
                    break;
                }
                let start_less : usize = curl_list.pop_back().unwrap();
                let start_clam : usize = curl_list.pop_back().unwrap();
                let start_para : usize = curl_list.pop_back().unwrap();
                println!("");
                println!("start para {} clam {} less {}", start_para, start_clam, start_less);
                if start_para != open_para {
                    syntax_error = true;
                    break;
                }
                if start_clam != open_clam {
                    syntax_error = true;
                    break;
                }
                if start_less != open_less {
                    syntax_error = true;
                    break;
                }
                open_curl -= 1;
            }
            if c == '>' {
                if open_less == 0 {
                    syntax_error = true;
                    break;
                }
                let start_curl : usize = less_list.pop_back().unwrap();
                let start_clam : usize = less_list.pop_back().unwrap();
                let start_para : usize = less_list.pop_back().unwrap();
                println!("");
                println!("start para {} clam {} curl {}", start_para, start_clam, start_curl);
                if start_para != open_para {
                    syntax_error = true;
                    break;
                }
                if start_clam != open_clam {
                    syntax_error = true;
                    break;
                }
                if start_curl != open_curl {
                    syntax_error = true;
                    break;
                }
                open_less -= 1;
            }

            print!(".para{}", open_para);
            print!(".clam{}", open_clam);
            print!(".curl{}", open_curl);
            print!(".less{};", open_less);
            println!("");

            open_para_vec[pos] = open_para;
            open_clam_vec[pos] = open_clam;
            open_curl_vec[pos] = open_curl;
            open_less_vec[pos] = open_less;

            pos += 1;
        } // for

        println!("");
        if syntax_error {
            println!("Syntax Error");
            if last_char == ')' {
                score += 3;
            }
            if last_char == ']' {
                score += 57;
            }
            if last_char == '}' {
                score += 1197;
            }
            if last_char == '>' {
                score += 25137;
            }
            nsyntax += 1;
        }




        // part2
        let mut auto = String::new();
        // consider correct lines
        if syntax_error == false {

            let mut close_para : usize = 0;
            let mut close_clam : usize = 0;
            let mut close_curl : usize = 0;
            let mut close_less : usize = 0;

            // walk again, back wards
            for (j, c) in bin.chars().rev().enumerate() {
                // do something with character `c` and index `i`
                print!("char2[{}] {}", j, c);


                print!(".parar{}", close_para);
                print!(".clamr{}", close_clam);
                print!(".curlr{}", close_curl);
                print!(".lessr{};", close_less);
                println!("");

                // ignore already matched closing chars:  ) ] } >

                // count
                if c == ')' {
                    close_para += 1;
                }
                if c == ']' {
                    close_clam += 1;
                }
                if c == '}' {
                    close_curl += 1;
                }
                if c == '>' {
                    close_less += 1;
                }

                // close
                if c == '(' {
                    if close_para > 0 {
                        close_para -= 1;
                    }
                    else {
                        auto.push_str( ")" );
                    }
                }
                if c == '[' {
                    if close_clam > 0 {
                        close_clam -= 1;
                    }
                    else {
                        auto.push_str( "]" );
                    }
                }
                if c == '{' {
                    if close_curl > 0 {
                        close_curl -= 1;
                    }
                    else {
                        auto.push_str( "}" );
                    }
                }
                if c == '<' {
                    if close_less > 0 {
                        close_less -= 1;
                    }
                    else {
                        auto.push_str( ">" );
                    }
                }

            }
            println!("AUTO={}", auto);
            // calc auto score
            let mut autoscore : usize = 0;
            // walk again, back wards
            for (k, c) in auto.chars().enumerate() {
                // do something with character `c` and index `i`
                print!("auto[{}] {}", k, c);
                autoscore *= 5;
                if c == ')' {
                    autoscore += 1;
                }
                if c == ']' {
                    autoscore += 2;
                }
                if c == '}' {
                    autoscore += 3;
                }
                if c == '>' {
                    autoscore += 4;
                }
            }
            println!("autoscore {}", autoscore);
            auto_vec.push( autoscore );
        }


    } // for all lines

    auto_vec.sort();
    let auto_len = auto_vec.len();
    let middle : usize = auto_vec[ auto_len / 2 ];
    for a in auto_vec {
        println!("a {}", a);
    }

    println!("total_syntax {} score {} autolen {} middle {}", nsyntax, score, auto_len, middle);

}
