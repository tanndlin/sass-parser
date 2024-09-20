use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn parse_args(args: Vec<String>) -> (String, String) {
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let file_name = args[1].clone();
    if !std::path::Path::new(&file_name)
        .extension()
        .map_or(false, |ext| ext.eq_ignore_ascii_case("scss"))
    {
        eprintln!("Expected a .scss file, got {file_name}");
        std::process::exit(1);
    }

    let result_file_name = file_name.replace(".scss", ".css");

    (file_name, result_file_name)
}

pub fn read_file(file_name: &str) -> String {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);
    let mut contents = String::new();
    for line in reader.lines() {
        contents.push_str(format!("{}\n", line.unwrap()).as_str());
    }
    contents
}
