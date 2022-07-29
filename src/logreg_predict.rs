mod feature;

use crate::feature::Feature;
use std::{env, fs, process};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = match fs::read_to_string(args[1].clone()) {
        Ok(x) => x,
        Err(_) => {
            println!("Something went wrong when reading the file");
            process::exit(1);
        }
    };
    let features = match Feature::new_and_init(contents) {
        Ok(all_features) => all_features,
        Err(e) => {
            println!("{e}");
            process::exit(1);
        }
    };
    let students_nb = features[0].get_count();
    let mut i = 0;
    while i < students_nb {
        let mut score = 0.0; //0 c'est theta0 pas compris encore lol mdr trop marrant
        features.iter().for_each(|x| {
            match x.get_values()[i].parse::<f64>() {
                Ok(val) => {
                    score += val * 1.0; //1 c'est theta/poids a determiner dans train
                },
                _ => ()
            }
        });
        i += 1;
        println!("{}", sigmoid(score));
    }
}

fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}
