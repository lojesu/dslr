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
}

fn histogram(features: Vec<Feature>, all_rsd: Vec<f32>) {
    let root = BitMapBackend::new("graphics/histogram.png", (640, 480)).into_drawing_area();
        root.fill(&WHITE).unwrap();

    let mut names: Vec<String> = Vec::new();
    features.iter().for_each(|x| {
        names.push(x.get_name());
    });
    let mut ctx = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 30)
        .set_label_area_size(LabelAreaPosition::Right, 30)
        .set_label_area_size(LabelAreaPosition::Bottom, 30)
        .caption("relative standard deviation (RSD)", ("sans-serif", 20))
        .build_cartesian_2d(0..(all_rsd.len() - 1), 0..100)
        .unwrap();
    ctx.configure_mesh().draw().unwrap();
    
    ctx.draw_series((0..).zip(all_rsd.iter()).map(|(x, y)| {
        let mut bar = Rectangle::new([(x, 0), (x, *y as i32)], RED.filled());
        bar.set_margin(0, 0, 2, 2);
        bar
    }))
    .unwrap();
}
