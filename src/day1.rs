use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use regex::Regex;


pub fn exec_day1() -> Result<(), Error> {
    let path = "day1/input-test-part-1.txt";
    // let path = "day1/input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let re = Regex::new(r"[0-9]{1}").unwrap();

    let mut suma: i32 = 0;

    for line in buffered.lines() {
        println!("{:?}", line);

        let cadena = line.unwrap(); 
        
        let numeros: Vec<&str> = re.find_iter(&cadena).map(|m| m.as_str()).collect();
        
        println!("{:?}", numeros);


        // println!("Firt element {:?}: ", numeros.pop());

        
        // debug: per comprovar si l'agafem bé
        // let primer_caracter = numeros[0].parse::<i32>();
        // let ultim_caracter = numeros[numeros.len()-1].parse::<i32>();
        // println!("primer i ultim {:?} {:?}", primer_caracter, ultim_caracter);

        let combi = [numeros[0],numeros[numeros.len()-1]].join("").parse::<i32>();
        // println!("{:?}", combi);

//        suma = &suma + &combi.unwrap();
    }
    println!("{:?}", suma);

    //    hay.find(&re)
//    for line in buffered.lines() {
//        let cadena = line?;
//        println!("{}", cadena);
//        
        // let first_digit = cadena.find(&re);
//    }
let re = Regex::new(r"[0-9]{1}|eight|one|two|three|four|five|six|seven|nine").unwrap();

let mut sum_calibration: i32 = 0;
// let path = "day1/input-test-part-2.txt";
let path = "day1/input.txt";

let input = File::open(path)?;
let buffered = BufReader::new(input);

fn change_number(numero: &str) -> &str {
    if numero == "one" { "1" }
    else if numero == "two" { "2" }
    else if numero == "three" { "3" }
    else if numero == "four" { "4" }
    else if numero == "five" { "5" }
    else if numero == "six" { "6" }
    else if numero == "seven" { "7" }
    else if numero == "eight" { "8" }
    else if numero == "nine" { "9" }
    else { numero }
}

/*
fn rem_n_chars(value: &str, longitud: usize) -> &str {
    let mut chars: String = value.chars().skip(longitud).collect();
    chars.as_str();
}*/


    Ok(())

}
