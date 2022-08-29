mod interpreter;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(Path::new(&args[1])).expect("[ERROR] File Not Found");
    let code: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.expect("[ERROR] Could Not Read File"))
        .collect();

    interpreter::evaluate(&code);
}
