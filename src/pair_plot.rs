use crate::feature::Feature;
use plotters::prelude::*;

pub fn pair_plot(features: Vec<Feature>, houses: Vec<&str>) {
    let root = BitMapBackend::new("graphics/pair_plot.png", (1366, 768)).into_drawing_area();

    let nb_feature = features.len() / 2;
    let areas = root.split_evenly((nb_feature, nb_feature));

    for (id, area) in areas.into_iter().enumerate() {
        let feature_x = id % nb_feature;
        let feature_y = id / nb_feature;
        area.fill(&WHITE).unwrap();
        let mut ctx = ChartBuilder::on(&area)
        .set_label_area_size(LabelAreaPosition::Left, 10.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 10.0)
        .build_cartesian_2d((features[feature_x].get_min().unwrap() as f32)..(features[feature_x].get_max().unwrap() as f32), (features[feature_y].get_min().unwrap() as f32)..(features[feature_y].get_max().unwrap() as f32))
        .unwrap();
        ctx.configure_mesh().draw().unwrap();

        if feature_y == 0 {
            let title = features[feature_x].get_name();
            let mut ctx = ChartBuilder::on(&area)
            .set_label_area_size(LabelAreaPosition::Left, 0)
            .set_label_area_size(LabelAreaPosition::Bottom, 0)
            .caption(title, ("sans-serif", 10))
            .build_cartesian_2d((features[feature_x].get_min().unwrap() as f32)..(features[feature_x].get_max().unwrap() as f32), (features[feature_y].get_min().unwrap() as f32)..(features[feature_y].get_max().unwrap() as f32))
            .unwrap();
            ctx.configure_mesh().draw().unwrap();
        } else if feature_x == 0 {
            let title = features[feature_y].get_name();
            let mut ctx = ChartBuilder::on(&area)
            .set_label_area_size(LabelAreaPosition::Left, 0)
            .set_label_area_size(LabelAreaPosition::Bottom, 0)
            .caption(title, ("sans-serif", 10))
            .build_cartesian_2d((features[feature_x].get_min().unwrap() as f32)..(features[feature_x].get_max().unwrap() as f32), (features[feature_y].get_min().unwrap() as f32)..(features[feature_y].get_max().unwrap() as f32))
            .unwrap();
            ctx.configure_mesh().draw().unwrap();
        }
        else if feature_x != feature_y {
            let data1: Result<Vec<f32>, _> = features[feature_x].get_values().iter().map(|x| x.parse()).collect();
            let data2: Result<Vec<f32>, _> = features[feature_y].get_values().iter().map(|x| x.parse()).collect();
            match data1 {
                Ok(data1_val) => {
                    match data2 {
                        Ok(data2_val) => {
                            let mut true_data = Vec::new();
                            let mut i = 0;
                            while data1_val.get(i) != None {
                                match data1_val.get(i) {
                                    Some(x) => {
                                        match data2_val.get(i) {
                                            Some(y) => {
                                                if i < houses.len() {
                                                    match houses[i] {
                                                       "Gryffindor" => true_data.push((*x as f32, *y as f32, &RED)),
                                                        "Hufflepuff" => true_data.push((*x as f32, *y as f32, &YELLOW)),
                                                        "Slytherin" => true_data.push((*x as f32, *y as f32, &GREEN)),
                                                        _  => true_data.push((*x as f32, *y as f32, &BLUE)),
                                                    }
                                                } else {
                                                    true_data.push((*x as f32, *y as f32, &BLUE));
                                                }
                                            },
                                            _ => ()
                                        }
                                    },
                                    _ => ()
                                }
                                i += 1;
                            }
                            if features[feature_x].get_name() == "Defense Against the Dark Arts" && features[feature_y].get_name() == "Arithmancy" {
                                true_data.iter().for_each(|point| {
                                    println!("x = {} | y = {}", point.0, point.1);
                                });
                            }
                            ctx.draw_series(
                                true_data.iter().map(|point| Circle::new((point.0, point.1), 1.0, point.2))
                            ).unwrap();
                        },
                        _ => ()
                    }
                },
                _ => ()
            };
        } else {
            let mut ctx = ChartBuilder::on(&area)
            .set_label_area_size(LabelAreaPosition::Left, 0)
            .set_label_area_size(LabelAreaPosition::Bottom, 0)
            .build_cartesian_2d(0..(features[feature_x].get_count() as i32), (features[feature_x].get_min().unwrap() as i32)..(features[feature_x].get_max().unwrap() as i32))
            .unwrap();
            ctx.configure_mesh().draw().unwrap();
            let new_values: Result<Vec<f32>, _> = features[feature_x].get_values().iter().map(|x| x.parse()).collect();
            match new_values {
                Ok(data) => {
                    ctx.draw_series((0..).zip(data.iter()).map(|(x, y)| {
                        let mut bar = Rectangle::new([(x, 0), (x, *y as i32)], BLACK.filled());
                        bar.set_margin(0, 0, 1, 1);
                        bar
                    }))
                    .unwrap();
                },
                _ => ()
            }
        }
    }
}
