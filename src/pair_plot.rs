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

fn pair_plot(features: Vec<Feature>, houses: Vec<&str>) {
    let root = BitMapBackend::new("graphics/pair_plot.png", (1366, 768)).into_drawing_area();

    let nb_feature = features.len();
    let areas = root.split_evenly((nb_feature, nb_feature));

    for (id, area) in areas.into_iter().enumerate() {
        let feature_x = id % nb_feature;
        let feature_y = id / nb_feature;
        area.fill(&WHITE).unwrap();

        if feature_x != feature_y {
            let mut ctx = ChartBuilder::on(&area)
            .build_cartesian_2d((features[feature_x].get_min().unwrap() as f32)..(features[feature_x].get_max().unwrap() as f32), (features[feature_y].get_min().unwrap() as f32)..(features[feature_y].get_max().unwrap() as f32))
            .unwrap();
            ctx.configure_mesh().draw().unwrap();
            let data1: Vec<f32> = features[feature_x].get_values().iter().map(|x| x.parse().unwrap_or(f32::NAN)).collect();
            let data2: Vec<f32> = features[feature_y].get_values().iter().map(|x| x.parse().unwrap_or(f32::NAN)).collect();
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
                true_data.iter().map(|point| Circle::new((point.0, point.1), 2.0, point.2))
            ).unwrap();
        } else {
            let mut ctx = ChartBuilder::on(&area)
            .set_label_area_size(LabelAreaPosition::Left, 10)
            .set_label_area_size(LabelAreaPosition::Bottom, 10)
            .build_cartesian_2d(0..(features[feature_x].get_count() as i32), (features[feature_x].get_min().unwrap() as i32)..(features[feature_x].get_max().unwrap() as i32))
            .unwrap();
            ctx.configure_mesh().draw().unwrap();
            let data: Vec<f32> = features[feature_x].get_values().iter().map(|x| x.parse().unwrap_or(f32::NAN)).collect();
            ctx.draw_series((0..).zip(data.iter()).enumerate().map(|(i, (x, y))| {
                let mut bar = Rectangle::new([(x, 0), (x, *y as i32)], BLUE.filled());
                match houses[i] {
                   "Gryffindor" => bar = Rectangle::new([(x, 0), (x, *y as i32)], RED.filled()),
                    "Hufflepuff" => bar = Rectangle::new([(x, 0), (x, *y as i32)], YELLOW.filled()),
                    "Slytherin" => bar = Rectangle::new([(x, 0), (x, *y as i32)], GREEN.filled()),
                    _  => bar = Rectangle::new([(x, 0), (x, *y as i32)], BLUE.filled()),
                }
                
                bar.set_margin(0, 0, 1, 1);
                bar
            }))
            .unwrap();
        }
    }
}
