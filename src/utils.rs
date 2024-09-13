use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_file(file_name: &str) -> String {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);
    let mut contents = String::new();
    for line in reader.lines() {
        contents.push_str(&line.unwrap());
        contents.push('\n');
    }
    contents
}
