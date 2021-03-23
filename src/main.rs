use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::collections::HashMap;

fn main() {
    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("Error reading line!");
    let path = path.trim();
    read_file(path);
}

fn read_file(name: &str) {
    let mut file = File::open(name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let result = parse_csv(&contents);
    write_to_csv(result);
    println!("{}", "Finished successfully!");
}

fn write_to_csv(content: Vec<Vec<String>>) -> Result<(), io::Error> {
    let mut file = File::create("output.csv").unwrap();

    let mut pos = 0;
    for row in content {
        for (idx, element) in row.iter().enumerate() {
            let bytes_written = file.write(element.as_bytes()).unwrap();
            pos += bytes_written;
            if idx < row.len() - 1 {
                let bytes_written = file.write(b",").unwrap();
                pos += bytes_written;
            }
        }

        let bytes_written = file.write(b"\n").unwrap();
        pos += bytes_written;
    }

    Ok(())
}

fn parse_csv(content: &String) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();
    let mut row: Vec<String> = Vec::new();
    let mut buf = vec![];
    for c in content.chars() {
        match c {
            '\n' => {
                row.push(buf.clone().iter().collect::<String>());
                buf = vec![];
                result.push(row.clone());
                row = Vec::new();
            },
            ',' => {
                row.push(buf.clone().iter().collect::<String>());
                buf = vec![];
            },
            _ => {
                buf.push(c);
            }
        }
    }

    row.push(buf.clone().iter().collect::<String>());
    result.push(row);

    result
}