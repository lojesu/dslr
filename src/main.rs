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
    let mut feature_std_min = features.get(0).unwrap();
    let mut all_std: Vec<f32> = Vec::new();
    features.iter().for_each(|x| {
        match x.get_std() {
            Some(x_std) => {
                all_std.push(x_std);
                match feature_std_min.get_std() {
                    Some(stdmin_std) => {
                        if x_std < stdmin_std {
                            feature_std_min = x;
                        }
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    });
    histogram(feature_std_min.clone(), all_std);

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
