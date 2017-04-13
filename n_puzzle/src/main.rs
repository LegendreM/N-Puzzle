mod n_puzzle;
mod parser;

use std::env;
use n_puzzle::n_puzzle;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            n_puzzle(&args[1]);
        },
        _ => {            
            println!("usage: n_puzzle <file_path>");
        }
    }
}
