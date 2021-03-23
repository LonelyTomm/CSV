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
    println!("{}", "HI");
}

fn write_to_csv(content: HashMap<u64, HashMap<u64, String>>) -> Result<(), io::Error> {
    let mut file = File::create("output.csv").unwrap();

    let mut pos = 0;
    for index_row in 0..content.len() {
        let row = content.get(&(index_row as u64)).unwrap();
        for index_el in 0..row.len() {
            let element = row.get(&(index_el as u64)).unwrap();
            let bytes_written = file.write(element.as_bytes()).unwrap();
            pos += bytes_written;
            if index_el < row.len() - 1 {
                let bytes_written = file.write(b",").unwrap();
                pos += bytes_written;
            }
        }

        let bytes_written = file.write(b"\n").unwrap();
        pos += bytes_written;
    }

    Ok(())
}

fn parse_csv(content: &String) -> HashMap<u64, HashMap<u64, String>> {
    let mut result: HashMap<u64, HashMap<u64, String>> = HashMap::new();
    let mut row: HashMap<u64, String> = HashMap::new();
    let mut index_overall = 0;
    let mut index_row = 0;
    let mut buf = vec![];
    for c in content.chars() {
        match c {
            '\n' => {
                row.insert(index_row, buf.clone().iter().collect::<String>());
                buf = vec![];
                index_row = 0;
                result.insert(index_overall, row.clone());
                row = HashMap::new();
                index_overall += 1;
            },
            ',' => {
                row.insert(index_row, buf.clone().iter().collect::<String>());
                buf = vec![];
                index_row += 1;
            },
            _ => {
                buf.push(c);
            }
        }
    }

    row.insert(index_row, buf.clone().iter().collect::<String>());
    buf = vec![];
    result.insert(index_overall, row);
    row = HashMap::new();

    result
}