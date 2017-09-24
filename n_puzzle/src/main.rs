use std::env;
use std::fs::File;
use std::io::Read;
use std::str::Lines;
use std::process;

mod board;

use board::Board;
use board::BoardType;

fn read_file(filename: &String) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    // println!("With text:\n{}", contents);
    contents
}

fn remove_comments(lines: Lines) -> Vec<&str> {
    let mut ret: Vec<&str> = Vec::new();

    for line in lines {
        match line.chars().nth(0) {
            Some('#') => continue,
            Some(_) => {
                match line.split("#").next() {
                    Some(s) => ret.push(s),
                    None => continue,
                }
            },
            _ => continue,
        }
    }
    ret
}

fn is_teals_valid(board: &Vec<Vec<u32>>, line_count: usize) -> bool {
    let my_vec = board.clone();

    let mut flatted: Vec<_> = my_vec.iter().flat_map(|v| v.iter()).collect();
    let before_dedup_len = flatted.len();

    flatted.sort();
    flatted.dedup();

    if before_dedup_len == flatted.len() {
        return !flatted.iter().any(|v| v >= &&(before_dedup_len as u32));
    } else {
        return false;
    }
}

fn parse_file(content: &String) -> Result<Vec<Vec<u32>>, String> {
    let lines = content.lines();
    let str_board = remove_comments(lines);
    // println!("Without comments:\n{:?}", str_board);
    let line_count;
    match str_board[0].parse::<usize>() {
        Ok(n) => line_count = n,
        Err(e) => return Err(format!("error parsing line count: {:?}", e)),
    }
    // println!("line count: {:?}", line_count);
    if str_board.len() != line_count + 1 {
        return Err(format!("error parsing line count: line count != {:?}", line_count));
    }
    let mut board: Vec<Vec<u32>> = Vec::new(); 
    for line in str_board.iter().skip(1) {
        let board_line: Result<Vec<_>, _> = line.split_whitespace()
                .map(|s| s.parse())
                .collect();
        match board_line {
            Ok(v) => {
                    if v.len() == line_count {
                        board.push(v);
                    }
                    else {
                        return Err(format!("error parsing line count: col count != {:?}", line_count))
                    }
                },
            Err(e) => return Err(format!("error parsing line count: {}", e)),
        }
    }
    // println!("board: {:?}", board);
    if is_teals_valid(&board, line_count) {
        return Ok(board);
    }
    return Err(format!("error parsing teals"));
}

// fn get_snail_order(board: &Vec<Vec<u32>>) -> Vec<u32> {
//     let mut snail: Vec<u32> = Vec::new();
//     let mut min = 0;
//     let mut max = board.len();

//     while min < max {
//         for x in min..max {
//             snail.push(board[min][x]);
//         }
//         for y in (min + 1)..(max - 1) {
//             snail.push(board[y][max - 1])
//         }
//         for x in (min..max).rev() {
//             snail.push(board[max - 1][x]);
//         }
//         for y in ((min + 1)..(max - 1)).rev() {
//             snail.push(board[y][min])
//         }
//         min += 1;
//         max -= 1;
//     }
//     snail.dedup();
//     snail
// }

// fn linear_goal(len: u32) -> Vec<u32> {
//     (0..len).collect()
// }

// fn snail_goal(len: u32) -> Vec<u32> {
//     let lin: Vec<u32> = (0..len).collect();
//     let mut snail: Vec<u32> = vec![1; 10];
// }

// fn is_puzzle_solvable(board: Vec<Vec<u32>>) {
//     let snail = get_snail_order(&board);
//     let goal = goal(snail.len() as u32);



// }

fn failable_main() -> Result<(), Box<::std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let contents = read_file(&args[1]);
        if contents.is_empty() {
            return Err(format!("Empty file").into());
        }
        let board_vec: Vec<Vec<u32>> = parse_file(&contents)?;
        let line_size = board_vec.len();
        let board_vec: Vec<u32> = board_vec.into_iter().flat_map(|v| v.into_iter()).collect();
        let board = Board::new(board_vec.into_boxed_slice(), line_size, BoardType::Snail);
        println!("board:\nlinear: {:?}\ntemplated: {:?}", board.linear(), board.templated());
        // println!("board: {:?}", get_snail_order(&board));

    }
    else {
        return Err(format!("No args :(").into());
    }
    Ok(())
}

fn main() {
    if let Err(e) = failable_main() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
