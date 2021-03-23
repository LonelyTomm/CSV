use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::collections::HashMap;

fn main() {
    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("Ошибка считанных данных");
    let path = path.trim();
    read_file(path);
}

fn read_file(name: &str) {
    let mut file = File::open(name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let result = parse_csv(&contents);
    println!("{}", "HI");
}

fn parse_csv(content: &String) -> HashMap<u64, HashMap<u64, String>> {
    let mut result: HashMap<u64, HashMap<u64, String>> = HashMap::new();
    let mut row: HashMap<u64, String> = HashMap::new();
    let mut indexOverall = 0;
    let mut indexRow = 0;
    let mut buf = vec![];
    for c in content.chars() {
        match c {
            '\n' => {
                row.insert(indexRow, buf.clone().iter().collect::<String>());
                buf = vec![];
                indexRow = 0;
                result.insert(indexOverall, row.clone());
                row = HashMap::new();
                indexOverall += 1;
            },
            ',' => {
                row.insert(indexRow, buf.clone().iter().collect::<String>());
                buf = vec![];
                indexRow += 1;
            },
            _ => {
                buf.push(c);
            }
        }
    }

    row.insert(indexRow, buf.clone().iter().collect::<String>());
    buf = vec![];
    indexRow = 0;
    result.insert(indexOverall, row);
    row = HashMap::new();
    indexOverall += 1;

    return result;
}