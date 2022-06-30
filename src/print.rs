use crate::feature::Feature;
use colored::Colorize;

fn common_print(features: &Vec<Feature>, feat: &str) {
    print!("{:<10}", feat.yellow());
    features.iter().for_each(|x| {
        match feat {
            "name" => {
                print!("{:<30}", x.get_name());
            },
            "count" => {
                print!("{:<30}", x.get_count());
            },
            "mean" => {
                match x.get_mean() {
                    Some(value) => print!("{:<30}", value),
                    None => print!("{:<30}", "/"),
                }
            },
            "std" => {
                match x.get_std() {
                    Some(value) => print!("{:<30}", value),
                    None => print!("{:<30}", "/"),
                }
            },
            "min" => {
                match x.get_min() {
                    Some(value) => print!("{:<30}", value),
                    None => print!("{:<30}", "/"),
                }
            },
            "25%" => {
                match x.get_p25() {
                    Some(value) => print!("{:<30}", value),
                    None => print!("{:<30}", "/"),
                }
            },
            "50%" => {
                match x.get_p50() {
                    Some(value) => print!("{:<30}", value),
                    None => print!("{:<30}", "/"),
                }
            },
            "75%" => {
                match x.get_p75() {
                    Some(value) => print!("{:<30}", value),
                    None => print!("{:<30}", "/"),
                }
            },
            "max" => {
                match x.get_max() {
                    Some(value) => print!("{:<30}", value),
                    None => print!("{:<30}", "/"),
                }
            },
            "unique" => {
                match x.get_unique() {
                    Some(value) => print!("{:<30}", value),
                    None => print!("{:<30}", "/"),
                }
            },
            "top" => {
                match x.get_top() {
                    Some(value) => print!("{:<30}", value.0),
                    None => print!("{:<30}", "/"),
                }
            },
            "freq" => {
                match x.get_top() {
                    Some(value) => print!("{:<30}", value.1),
                    None => print!("{:<30}", "/"),
                }
            },
            _ => {
                println!("unexpected error ! 'feat' doesn't exist");
                unreachable!();
            }
        }
    });
    print!("\n");
}

//commandor print function for describe
pub fn print_features(features: &Vec<Feature>) {
    common_print(features, "name");
    common_print(features, "count");
    common_print(features, "mean");
    common_print(features, "std");
    common_print(features, "min");
    common_print(features, "25%");
    common_print(features, "50%");
    common_print(features, "75%");
    common_print(features, "max");
    common_print(features, "unique");
    common_print(features, "top");
    common_print(features, "freq");
}
