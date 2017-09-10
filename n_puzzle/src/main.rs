use std::env;
use std::fs::File;
use std::io::Read;
use std::str::Lines;

fn read_file(filename: &String) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
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

fn parse_file(content: &String) {
    let lines = content.lines();
    println!("Without comments:\n{:?}", remove_comments(lines));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let contents = read_file(&args[1]);
            parse_file(&contents);
        },
        _ => println!("No args :("),
    }
}
