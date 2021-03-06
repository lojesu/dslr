#![allow(dead_code)]

mod feature;

use crate::feature::Feature;
use colored::Colorize;
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
    print_features(&features);
}

fn print_features(features: &Vec<Feature>) {
    println!("{:<32}{:<15}{:<15}{:<15}{:<15}{:<15}{:<15}{:<15}{:<15}{:<15}{:<15}{:<15}",
        "name".blue().bold(),
        "count".blue().bold(),
        "mean".blue().bold(),
        "std".blue().bold(),
        "min".blue().bold(),
        "max".blue().bold(),
        "p25".blue().bold(),
        "p50".blue().bold(),
        "p75".blue().bold(),
        "unique".blue().bold(),
        "top".blue().bold(),
        "freq".blue().bold(),
    );
    features.iter().for_each(|x| {
        print_line(x.clone());
    });
}

fn print_line(feat: Feature) {
    print!("{:<32}", feat.get_name().blue().bold());
    print!("{:<15}", feat.get_count());
    match feat.get_mean() {
        Some(value) => print!("{:<15}", value),
        _ => print!("{:<15}", "/".dimmed()),
    }
    match feat.get_std() {
        Some(value) => print!("{:<15}", value),
        _ => print!("{:<15}", "/".dimmed()),
    }
    match feat.get_min() {
        Some(value) => print!("{:<15}", value),
        _ => print!("{:<15}", "/".dimmed()),
    }
    match feat.get_max() {
        Some(value) => print!("{:<15}", value),
        _ => print!("{:<15}", "/".dimmed()),
    }
    match feat.get_p25() {
        Some(value) => print!("{:<15}", value),
        _ => print!("{:<15}", "/".dimmed()),
    }
    match feat.get_p50() {
        Some(value) => print!("{:<15}", value),
        _ => print!("{:<15}", "/".dimmed()),
    }
    match feat.get_p75() {
        Some(value) => print!("{:<15}", value),
        _ => print!("{:<15}", "/".dimmed()),
    }
    match feat.get_unique() {
        Some(value) => print!("{:<15}", value),
        _ => print!("{:<15}", "/".dimmed()),
    }
    match feat.get_top() {
        Some(value) => {
            print!("{:<15}", value.0);
            print!("{:<15}", value.1);
        },
        _ => {
            print!("{:<15}", "/".dimmed());
            print!("{:<15}", "/".dimmed());
        }
    }
    print!("\n");
}
