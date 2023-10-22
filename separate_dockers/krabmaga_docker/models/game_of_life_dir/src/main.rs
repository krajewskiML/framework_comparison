mod model;
mod visualization;

use clap::{Arg, Command};
use model::board::Board;
use krabmaga::simulate;

//#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
        // read in arguments
    let matches = Command::new("Game of Life")
        .version("0.1.0")
        .author("Maciej Krajewski")
        .about("Approach to Game of Life")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .help("A cool file"))
        .arg(Arg::new("steps")
            .short('s')
            .long("steps")
            .value_parser(clap::value_parser!(u64))
            .help("A cool step"))
        .get_matches();

    // get the map
    let default_file = "input.txt".to_string();
    let myfile: &String = matches.get_one::<String>("file").unwrap_or(&default_file);
    let steps = *matches.get_one::<u64>("steps").unwrap_or(&10000u64);

    // read csv file, it consists of 0 and 1 so load it into 2d array of bools it has no header, we start from 0,0
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(myfile)
        .unwrap();
    let mut map: Vec<Vec<bool>> = Vec::new();
    for result in rdr.records() {
        let record = result.unwrap();
        let mut row: Vec<bool> = Vec::new();
        for cell in record.iter() {
            if cell == "0" {
                row.push(false);
            } else {
                row.push(true);
            }
        }
        map.push(row);
    }
    let forest = Board::new(map);
    let _ = simulate!(forest, steps, 1, false);
}

// fn main() {
//     // read in arguments
//     let matches = Command::new("Game of Life")
//         .version("0.1.0")
//         .author("Maciej Krajewski")
//         .about("Approach to Game of Life")
//         .arg(Arg::new("file")
//             .short('f')
//             .long("file")
//             .help("A cool file"))
//         .get_matches();
//
//     // get the map
//     let default_file = "input.txt".to_string();
//     let myfile: &String = matches.get_one::<String>("file").unwrap_or(&default_file);
//
//     // read csv file, it consists of 0 and 1 so load it into 2d array of bools it has no header, we start from 0,0
//     let mut rdr = csv::ReaderBuilder::new()
//         .has_headers(false)
//         .from_path(myfile)
//         .unwrap();
//     let mut map: Vec<Vec<bool>> = Vec::new();
//     for result in rdr.records() {
//         let record = result.unwrap();
//         let mut row: Vec<bool> = Vec::new();
//         for cell in record.iter() {
//             if cell == "0" {
//                 row.push(false);
//             } else {
//                 row.push(true);
//             }
//         }
//         map.push(row);
//     }
//
//
//     // print first map
//     println!("Initial map:");
//     for row in map.iter() {
//         for cell in row.iter() {
//             if *cell {
//                 print!("1");
//             } else {
//                 print!("0");
//             }
//         }
//         println!();
//     }
// }