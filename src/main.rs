use std::fs::File;
use std::io::{self, Read, BufRead, BufReader};
use std::num::ParseIntError;
use std::path::PathBuf;
use std::{fmt, process};
use std::str::FromStr;

use structopt::StructOpt;

use n_puzzle::{Board, Solver, Tile};
use n_puzzle::{Manhattan, Dijkstra, Euclidean, MissPlaced, OutOfRaw};

#[derive(Debug)]
enum Error {
    PuzzleInvalidTiles,
    PuzzleMissingSize,
    PuzzleInvalidNumberOfTiles,
    PuzzleInvalidNumber(ParseIntError),
    IoError(io::Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::IoError(error)
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Error {
        Error::PuzzleInvalidNumber(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::PuzzleInvalidTiles => write!(f, "invalid tiles"),
            Error::PuzzleMissingSize => write!(f, "missing puzzle size"),
            Error::PuzzleInvalidNumberOfTiles => write!(f, "invalid number of tiles"),
            Error::PuzzleInvalidNumber(error) => write!(f, "invalid tile number; {}", error),
            Error::IoError(error) => write!(f, "io error; {}", error),
        }
    }
}

impl std::error::Error for Error { }

fn is_board_valid(numbers: &[Tile]) -> bool {
    let sort_dedup_numbers = {
        let mut numbers = numbers.to_vec();
        numbers.sort_unstable();
        numbers.dedup();
        numbers
    };

    if sort_dedup_numbers.len() != numbers.len() {
        return false
    }

    if sort_dedup_numbers.get(0) != Some(&0) {
        return false
    }

    for array in sort_dedup_numbers.windows(2) {
        if array[1] != array[0] + 1 {
            return false
        }
    }

    true
}

fn no_comment(string: &str) -> Option<&str> {
    string.split('#').next().filter(|s| !s.trim().is_empty())
}

fn read_board<R: Read>(read: R) -> Result<(Box<[Tile]>, usize), Error> {
    let read = BufReader::new(read);
    let mut lines = read.lines();

    // retrieve the puzzle size
    let mut size = None;
    for line in &mut lines {
        let line = line?;
        if let Some(number_part) = no_comment(&line) {
            let trimmed = number_part.trim();
            if !trimmed.is_empty() {
                size = Some(usize::from_str(trimmed)?);
                break;
            }
        }
    }

    let size = match size {
        Some(size) => size,
        None => return Err(Error::PuzzleMissingSize),
    };

    // retrieve the tiles numbers
    let mut numbers = Vec::with_capacity(size * size);
    for line in &mut lines {
        let line = line?;
        if let Some(tiles_part) = no_comment(&line) {
            let prev_len = numbers.len();

            for number_part in tiles_part.split_whitespace() {
                let number = Tile::from_str(number_part)?;
                numbers.push(number);
            }

            if numbers.len() - prev_len != size {
                return Err(Error::PuzzleInvalidNumberOfTiles)
            }
        }
    }

    if numbers.len() != size * size {
        return Err(Error::PuzzleInvalidNumberOfTiles)
    }

    if !is_board_valid(&numbers) {
        return Err(Error::PuzzleInvalidTiles)
    }

    Ok((numbers.into_boxed_slice(), size))
}

#[derive(Debug, StructOpt)]
#[structopt(name = "n-puzzle", about = "A* algorithm to solve npuzzles")]
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

fn failable_main() -> Result<(), Box<std::error::Error>> {
    let opt = Opt::from_args();

    println!("Value for input: {:?}", opt.input);
    println!("Value for expected: {:?}", opt.expected);
    println!("Value for heuristic: {}", opt.heuristic);

    let input = {
        let input_file = File::open(&opt.input)?;
        let (input_numbers, input_size) = read_board(input_file)?;
        Board::new(input_numbers, input_size)
    };

    let expected = {
        let expected_file = File::open(&opt.expected)?;
        let (expected_numbers, expected_size) = read_board(expected_file)?;
        Board::new(expected_numbers, expected_size)
    };

    match Solver::new(input, expected) {
        Ok(solver) => {
            let result = match opt.heuristic.as_str() {
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
