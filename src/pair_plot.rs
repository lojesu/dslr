use crate::feature::Feature;
use plotters::prelude::*;

pub fn pair_plot(features: Vec<Feature>, houses: Vec<&str>) {
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
            .build_cartesian_2d(0..(features[feature_x].get_count() as i32), (features[feature_x].get_min().unwrap() as i32)..(features[feature_x].get_max().unwrap() as i32))
            .unwrap();
            ctx.configure_mesh().draw().unwrap();
            let data: Vec<f32> = features[feature_x].get_values().iter().map(|x| x.parse().unwrap_or(f32::NAN)).collect();
            ctx.draw_series((0..).zip(data.iter()).map(|(x, y)| {
                let mut bar = Rectangle::new([(x, 0), (x, *y as i32)], BLACK.filled());
                bar.set_margin(0, 0, 1, 1);
                bar
            }))
            .unwrap();
        }
    }
}
