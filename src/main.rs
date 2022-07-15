mod describe;
mod print;
mod feature;
mod histogram;
mod pair_plot;

use describe::describe;
use histogram::histogram;
use pair_plot::pair_plot;

fn main() {
    let features = describe();
    // histogram part
    let mut all_rsd: Vec<f32> = Vec::new();
    features.iter().for_each(|x| {
        match x.get_std() {
            Some(x_std) => {
                match x.get_mean() {
                    Some(x_mean) => all_rsd.push(x_std / x_mean.abs() * 100.0),
                    _ => ()
                }
            },
            _ => ()
        }
    });
    histogram(features.clone(), all_rsd);

    // pair plot part
    let mut pair_plot_features = Vec::new();
    let mut houses: Vec<String> = Vec::new();
    features.iter().for_each(|x| {
        if x.get_name() == "Hogwarts House" {
            houses = x.get_values();
        }
        match x.get_std() {
            Some(_) => {
                if x.get_name() != "Index" {
                    pair_plot_features.push(x.clone());
                }
            },
            _ => ()
        }
    });
    let houses: Vec<&str> = houses.iter().map(|x| x as &str).collect();
    pair_plot(pair_plot_features, houses);
}
