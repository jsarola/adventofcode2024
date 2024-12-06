use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use regex::Regex;


pub fn exec_day2() -> Result<(), Error> {
    // let path = "day2/input-test-part-1.txt";
    let path = "day2/input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let re = Regex::new(r"(\s+)").unwrap();

    let mut suma_total_a: i32 = 0;
    let mut suma_total_b: i32 = 0;

    for line in buffered.lines() {
        // println!("{:?}", line);

        let linea = line.unwrap();
        let numeros: Vec<&str> = re.split(&linea).collect();

        // println!("{:?}", numeros);

        let mut numeros_i32: Vec<i32> = Vec::new() ;

        for element in numeros {
            numeros_i32.push(element.parse().unwrap());
        }

        // println!("{:?}", numeros_i32);
        //
        // starting calculus part 1
        
        let mut num_anterior: i32 = 0;
        let mut num_actual: i32 = 0;
        let mut counter = numeros_i32.len();
        let mut increase: bool = true;
        let mut safe: bool = true;

        num_anterior = numeros_i32[0];
        num_actual = numeros_i32[1];
        if num_actual > num_anterior {
            increase = true;
        } else {
            increase = false;
        }

        let mut index = 2;
        while index <= counter {
            // println!("increase: {:?} anterior: {:?} actual: {:?}", increase, num_anterior, num_actual);
            if increase {
                if num_actual <= num_anterior {
                   safe = false;
                   break;
                } else {
                    if num_actual - num_anterior > 3 {
                        safe = false;
                        break;
                    }
                }
            } else {
                if num_actual >= num_anterior {
                    safe = false;
                    break;
                } else {
                    if num_anterior - num_actual > 3 {
                        safe = false;
                        break;
                    }
                }
            }
            if index < counter {
                num_anterior = num_actual;
                num_actual = numeros_i32[index];
            }
            index = index + 1;
        }

        if safe {
            println!("{:?} is safe", linea);
            suma_total_a = suma_total_a + 1;
        } else {
            println!("{:?} is not safe", linea);
        }

        // finish calculus part 1


        // starting calculus part 2

        let mut num_anterior: i32 = 0;
        let mut num_actual: i32 = 0;
        let mut counter = numeros_i32.len();
        let mut increase: bool = true;
        let mut safe: bool = true;
        let mut badlevel: bool = false;

        num_anterior = numeros_i32[0];
        num_actual = numeros_i32[1];
        if num_actual > num_anterior {
            increase = true;
        } else {
            increase = false;
        }

        let mut index = 2;
        while index <= counter {
            // println!("increase: {:?} anterior: {:?} actual: {:?}", increase, num_anterior, num_actual);
            if increase {
                if num_actual <= num_anterior {
                    if badlevel {
                       safe = false;
                       break;
                    } else {
                       badlevel = true;
                    }
                } else {
                    if num_actual - num_anterior > 3 {
                        if badlevel {
                            safe = false;
                            break;
                        } else {
                            badlevel = true;
                        }
                    }
                }
            } else {
                if num_actual >= num_anterior {
                    if badlevel {
                        safe = false;
                        break;
                    } else {
                        badlevel = true;
                    }
                } else {
                    if num_anterior - num_actual > 3 {
                        if badlevel {
                            safe = false;
                            break;
                        } else {
                            badlevel = true;
                        }
                    }
                }
            }
            if index < counter {
                if badlevel == false {
                   num_anterior = num_actual;
                }
                num_actual = numeros_i32[index];
            }
            index = index + 1;
        }

        if safe {
            println!("{:?} is safe", linea);
            suma_total_b = suma_total_b + 1;
        } else {
            println!("{:?} is not safe", linea);
        }

        // finish calculus part 2
    }


    println!("Resultat apartat 1: {:?}", suma_total_a);

    println!("Resultat apartat 2: {:?}", suma_total_b);

    Ok(())

}
