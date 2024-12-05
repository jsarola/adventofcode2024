use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use regex::Regex;


pub fn exec_day1() -> Result<(), Error> {
    // let path = "day1/input-test-part-1.txt";
    let path = "day1/input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();

    let mut columna1a: Vec<i32> = Vec::new();
    let mut columna2a: Vec<i32> = Vec::new();
    let mut columna1b: Vec<i32> = Vec::new();
    let mut columna2b: Vec<i32> = Vec::new();

    for line in buffered.lines() {
        // println!("{:?}", line);
        if let Some(caps) = re.captures(&line.unwrap()) {
            columna1a.push(caps[1].parse::<i32>().unwrap());
            columna2a.push(caps[2].parse::<i32>().unwrap());
            
            columna1b.push(caps[1].parse::<i32>().unwrap());
            columna2b.push(caps[2].parse::<i32>().unwrap());

        }
    }

    columna1a.sort();
    columna2a.sort();

    let mut suma_total_a: i32 = 0;

    while let Some(number1) = columna1a.pop() {
        number2 = columna2a.pop().unwrap();
        distance = (number1 - number2).abs();

        suma_total_a = suma_total_a + distance;

        // println!("- Distance: {:?}", distance);
    }

    println!("Resultat apartat 1: {:?}", suma_total_a);

    let mut suma_total_b: i32 = 0;

    for numberx in columna1b {
        let count = columna2b.iter().filter(|&&y| y == numberx).count();

        // println!("{:?}", count);
        let count_i32 = count as i32;
        suma_total_b = suma_total_b + (numberx * count_i32)
    }

    println!("Resultat apartat 2: {:?}", suma_total_b);

    Ok(())

}
