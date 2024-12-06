use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use regex::Regex;

pub fn exec_day3() -> Result<(), Error> {
    let path = "day3/input-test-part-1.txt";
    // let path = "day3/input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut suma_total_a: i32 = 0;
    let mut suma_total_b: i32 = 0;

    // Calculus PART 1

    let re = Regex::new(r"(mul\((?<first>\d+),(?<second>\d+)\))").unwrap();

    for line in buffered.lines() {
        // println!("{:?}", line);

        let linea = line.unwrap();
        let numeros: Vec<(&str, &str)> = re.captures_iter(&linea).map(|caps| {
            let x = caps.name("first").unwrap().as_str();
            let y = caps.name("second").unwrap().as_str();
            (x, y)
        }).collect();

        // println!("{:?}", numeros);

        for tupla in numeros {
            // println!("{:?}", tupla);
            let valora: i32 = tupla.0.parse().unwrap();
            let valorb: i32 = tupla.1.parse().unwrap();

            let producte: i32 = valora * valorb;
            // println!("{:?}", producte);
            suma_total_a = suma_total_a + producte;
        }
    }

    // Calculus PART 2
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let re = Regex::new(r"(do\(\))|(don't\(\))").unwrap();

    for line in buffered.lines() {

        let linea = line.unwrap();
        let parts = re.split(&linea);
        println!("{:?}", parts.ok());

    }



    println!("Resultat apartat 1: {:?}", suma_total_a);

    println!("Resultat apartat 2: {:?}", suma_total_b);

    Ok(())

}
