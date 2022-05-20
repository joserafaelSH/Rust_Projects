use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

//
// fn ler_entrada(filename: &str) -> Vec<Vec<i32>> {
//     let file = File::open(filename).unwrap();
//     let reader = BufReader::new(file);

//     let mut cities: Vec<Vec<i32>> = Vec::new();

//     for (index, line) in reader.lines().enumerate() {
//         if index > 5 {
//             let line = line.unwrap();
//             if line.eq("EOF") {
//                 break;
//             }
//             let l_split: Vec<&str> = line.split(' ').collect();
//             cities.push(vec![
//                 l_split[1].parse::<i32>().unwrap(),
//                 l_split[2].parse::<i32>().unwrap(),
//             ]);
//         }
//     }

//     return cities;
// }

fn ler_entrada(filename: &str, cities: &mut [[i32; 2]; 90000]) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut count: usize = 0;

    for (index, line) in reader.lines().enumerate() {
        if index > 5 {
            let line = line.unwrap();
            if line.eq("EOF") {
                break;
            }
            let l_split: Vec<&str> = line.split(' ').collect();
            cities[count][0] = l_split[1].parse::<i32>().unwrap();
            cities[count][1] = l_split[2].parse::<i32>().unwrap();
            count += 1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let tam: &i32 = &args[2].parse::<i32>().unwrap();

    let mut cities: [[i32; 2]; 90000] = [[0; 2]; 90000];
    ler_entrada(filename, &mut cities);

    println!("Tamanho da entrada: {}", tam);
    let mut c: usize = 0;

    while c < *tam as usize {
        println!("{} {}", cities[c][0], cities[c][1]);
        c += 1;
    }
}
