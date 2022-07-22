mod feature;

use crate::feature::Feature;
use plotters::prelude::*;
use std::{env, fs, process};

fn main() {
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
    let tmp = features[1].get_values();
    let houses: Vec<&str> = tmp.iter().map(|x| x as &str).collect();
    scatter_plot(features[7].clone(), features[9].clone(), houses);
}

fn scatter_plot(feat_1: Feature, feat_2: Feature, houses: Vec<&str>) {
    println!("{} {}", feat_1.get_name(), feat_2.get_name());
    let root = BitMapBackend::new("graphics/scatter_plot.png", (960, 640))
        .into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Right, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 30)
    .caption("scatter_plot between Astronomy and Defense Against the Dark Arts", ("sans-serif", 20))
    .build_cartesian_2d((feat_1.get_min().unwrap() as f32)..(feat_1.get_max().unwrap() as f32), (feat_2.get_min().unwrap() as f32)..(feat_2.get_max().unwrap() as f32))
    .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let data1: Vec<f32> = feat_1.get_values().iter().map(|x| x.parse().unwrap_or(f32::NAN)).collect();
    let data2: Vec<f32> = feat_2.get_values().iter().map(|x| x.parse().unwrap_or(f32::NAN)).collect();

    let mut true_data = Vec::new();
    let mut i = 0;
    while i < data1.len() && i < data2.len() {
        match data1.get(i) {
            Some(x) => {
                match data2.get(i) {
                    Some(y) => {
                        if x.is_nan() == false && y.is_nan() == false {
                            if i < houses.len() {
                                match houses[i] {
                                   "Gryffindor" => true_data.push((*x, *y, &RED)),
                                    "Hufflepuff" => true_data.push((*x, *y, &YELLOW)),
                                    "Slytherin" => true_data.push((*x, *y, &GREEN)),
                                    _  => true_data.push((*x, *y, &BLUE)),
                                }
                            } else {
                                true_data.push((*x, *y, &BLUE));
                            }
                        }
                    },
                    _ => ()
                }
            },
            _ => ()
        }
        i += 1;
    }
    ctx.draw_series(
        true_data.iter().map(|point| Circle::new((point.0, point.1), 3.0, point.2))
    ).unwrap();
}
