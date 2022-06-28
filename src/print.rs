use crate::feature::Feature;

fn print_name(features: &Vec<Feature>) {
    print!("{:<10}", "name");
    features.iter().for_each(|x| print!("{:<30}", x.get_name()));
    print!("\n");
}

fn print_count(features: &Vec<Feature>) {
    print!("{:<10}", "count");
    features.iter().for_each(|x| print!("{:<30}", x.get_count()));
    print!("\n");
}

fn print_mean(features: &Vec<Feature>) {
    print!("{:<10}", "mean");
    features.iter().for_each(|x| {
        match x.get_mean() {
            Some(value) => print!("{:<30}", value),
            None => print!("{:<15}", "/"),
        }
    });
    print!("\n");
}

fn print_std(features: &Vec<Feature>) {
    print!("{:<10}", "std");
    features.iter().for_each(|x| {
        match x.get_std() {
            Some(value) => print!("{:<30}", value),
            None => print!("{:<15}", "/"),
        }
    });
    print!("\n");
}

//commandor print function for describe
pub fn print_features(features: Vec<Feature>) {
    print_name(&features);
    print_count(&features);
    print_mean(&features);
    print_std(&features);
}
