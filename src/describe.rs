use std::{env, fs, process};
use crate::print::print_features;
use crate::feature::Feature;

pub fn describe() -> Vec<Feature> {
    let args: Vec<String> = env::args().collect();
    let contents = match fs::read_to_string(args[1].clone()) {
        Ok(x) => x,
        Err(_) => {
            println!("Something went wrong when reading the file");
            process::exit(1);
        }
    };
    let mut features = match Feature::new_and_init(contents) {
        Ok(all_features) => all_features,
        Err(e) => {
            println!("{e}");
            process::exit(1);
        }
    };
    //println!("{features:?}");
    print_features(&features);
    features
}
