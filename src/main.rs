use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process;
use std::str::Lines;
use std::error::Error;

use structopt::StructOpt;

use n_puzzle::{Board, Solver, Tile};
use n_puzzle::{Manhattan, Dijkstra, Euclidean, MissPlaced, OutOfRaw};

fn read_file(filename: &Path) -> Result<String, String> {
    let f = File::open(filename);
    match f {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => Ok(contents),
                Err(e) => Err(format!("error in read file: {:?}", e))
            }
        },
        Err(e) => Err(format!("error in read file: {:?}", e))
    }

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

fn is_teals_valid(board: &Vec<Vec<Tile>>) -> bool {
    let my_vec = board.clone();

    let mut flatted: Vec<_> = my_vec.iter().flat_map(|v| v.iter()).collect();
    let before_dedup_len = flatted.len();

    flatted.sort();
    flatted.dedup();

    if before_dedup_len == flatted.len() {
        return !flatted.iter().any(|v| v >= &&(before_dedup_len as Tile));
    } else {
        return false;
    }
}

fn parse_file(content: &String) -> Result<Vec<Vec<Tile>>, String> {
    let lines = content.lines();
    let str_board = remove_comments(lines);
    let line_count;

    if str_board.len() == 0 {
        return Err(format!("file doesn't contain board"))
    }
    match str_board[0].parse::<usize>() {
        Ok(n) => line_count = n,
        Err(e) => return Err(format!("error parsing line count: {:?}", e)),
    }
    if str_board.len() != line_count + 1 {
        return Err(format!("error parsing line count: line count != {:?}", line_count));
    }
    let mut board: Vec<Vec<Tile>> = Vec::new();
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
    if is_teals_valid(&board) {
        return Ok(board);
    }
    return Err(format!("error parsing teals"));
}

#[derive(Debug, StructOpt)]
#[structopt(name = "n-puzzle", about = "A* algorithm to solve npuzzle")]
struct Opt {
    /// Input file which contains the npuzzle to solve
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Input file which contains the npuzzle expected solution
    #[structopt(parse(from_os_str))]
    expected: PathBuf,

    /// Heuristic used to solve npuzzle [manhattan, dijkstra, euclidean, miss_placed, out_of_raw]
    heuristic: String,
}

fn failable_main() -> Result<(), Box<Error>> {
    let opt = Opt::from_args();

    let input = opt.input;
    println!("Value for input: {:?}", input);

    let expected = opt.expected;
    println!("Value for expected: {:?}", expected);

    let heuristic = opt.heuristic;
    println!("Value for heuristic: {}", heuristic);

    let contents = read_file(&input)?;
    if contents.is_empty() {
        return Err(format!("Empty input file").into());
    }
    let expected = read_file(&expected)?;
    if expected.is_empty() {
        return Err(format!("Empty expected file").into());
    }
    let board_vec: Vec<Vec<Tile>> = parse_file(&contents)?;
    let line_size = board_vec.len();
    let board_vec: Vec<Tile> = board_vec.into_iter().flat_map(|v| v.into_iter()).collect();
    let board = Board::new(board_vec.into_boxed_slice(), line_size);

    let expected_vec: Vec<Vec<Tile>> = parse_file(&expected)?;
    let expected_line_size = expected_vec.len();
    let expected_vec: Vec<Tile> = expected_vec.into_iter().flat_map(|v| v.into_iter()).collect();
    let expected = Board::new(expected_vec.into_boxed_slice(), expected_line_size);

    match Solver::new(board, expected) {
        Ok(solver) => {
            let result = match heuristic.as_str() {
                "manhattan" => solver.solve::<Manhattan>(),
                "dijkstra" => solver.solve::<Dijkstra>(),
                "euclidean" => solver.solve::<Euclidean>(),
                "miss_placed" => solver.solve::<MissPlaced>(),
                "out_of_raw" => solver.solve::<OutOfRaw>(),
                _ => solver.solve::<Manhattan>(),
            };

            let (mem, time, moves) = result;
            println!("memory complexity: {}", mem);
            println!("time complexity: {}", time);
            println!("moves count: {}", moves.len());
            println!("moves:\r\n{:?}", moves);
        },
        Err(e) => println!("{}", e)
    }
    Ok(())
}

fn main() {
    if let Err(e) = failable_main() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
