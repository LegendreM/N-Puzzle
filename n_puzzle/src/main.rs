extern crate n_puzzle;

use std::env;
use std::fs::File;
use std::io::Read;
use std::str::Lines;
use std::process;

use n_puzzle::solver::{Board, Solver};

fn read_file(filename: &String) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

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
    let line_count;
    match str_board[0].parse::<usize>() {
        Ok(n) => line_count = n,
        Err(e) => return Err(format!("error parsing line count: {:?}", e)),
    }
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
    if is_teals_valid(&board, line_count) {
        return Ok(board);
    }
    return Err(format!("error parsing teals"));
}

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
        let board = Board::new(board_vec.into_boxed_slice(), line_size);

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
