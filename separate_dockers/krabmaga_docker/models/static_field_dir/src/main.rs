mod model;

use clap::{Arg, Command};

use krabmaga::simulate;

fn main() {
    let matches = Command::new("Game of Life")
        .version("0.1.0")
        .author("Maciej Krajewski")
        .about("Approach to Game of Life")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .help("File describing initial state"))
        .get_matches();

    let default_file = "input.txt".to_string();
    let myfile: &String = matches.get_one::<String>("file").unwrap_or(&default_file);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(myfile)
        .unwrap();

    let mut map: Vec<Vec<i8>> = Vec::new();
    for result in rdr.records() {
        let record = result.unwrap();
        let mut row: Vec<i8> = Vec::new();
        for cell in record.iter() {
            row.push(cell.parse::<i8>().unwrap());
        }
        map.push(row);
    }

    let mut board = model::board::Board::new(map);

    let steps = 1000;
    let _ = simulate!(board, u64::MAX, 1, false);
}

