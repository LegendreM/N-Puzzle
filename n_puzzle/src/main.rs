use std::env;
use std::fs::File;
use std::io::Read;

fn read_file(filename: &String) {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        2 => read_file(&args[1]),
        _ => println!("Hello, world!"),
    }
}
