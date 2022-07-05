use plotters::prelude::*;
use crate::feature::Feature;

pub fn histogram(feature: Feature, all_std: Vec<f32>) {
    let root = BitMapBackend::new("graphics/histogram.png", (640, 480)).into_drawing_area();
        root.fill(&WHITE).unwrap();
    let (upper, lower) = root.split_vertically((70).percent());

    let title = format!("values for {} feature", feature.get_name());
    let mut lower_chart = ChartBuilder::on(&lower)
        .set_label_area_size(LabelAreaPosition::Left, 30)
        .set_label_area_size(LabelAreaPosition::Right, 30)
        .set_label_area_size(LabelAreaPosition::Bottom, 30)
        .caption(&title, ("sans-serif", 20))
        .build_cartesian_2d(0..feature.get_count(), (feature.get_min().unwrap() as i32)..(feature.get_max().unwrap() as i32))
        .unwrap();

    lower_chart.configure_mesh().draw().unwrap();

    let new_values: Result<Vec<f32>, _> = feature.get_values().iter().map(|x| x.parse()).collect();

    match new_values {
        Ok(mut data) => {
            data.sort_by(|a, b| {
                match a.partial_cmp(b) {
                    Some(ord) => ord,
                    _ => {
                        println!("unexpected error when sort values");
                        unreachable!();
                    }
                }
            });
            lower_chart.draw_series((0..).zip(data.iter()).map(|(x, y)| {
                let mut bar = Rectangle::new([(x, 0), (x, *y as i32)], RED.filled());
                bar.set_margin(0, 0, 2, 2);
                bar
            }))
            .unwrap();
            },
        _ => ()
    }
    
    let mut all_std_clone = all_std.clone();
    all_std_clone.sort_by(|a, b| {
        match a.partial_cmp(b) {
            Some(ord) => ord,
            _ => {
                println!("unexpected error when sort values");
                unreachable!();
            }
        }
    });
    let pos: f32 = 50.0 / 100.0 * (all_std_clone.len() as f32 + 1.0);
    let mut upper_chart = ChartBuilder::on(&upper)
        .set_label_area_size(LabelAreaPosition::Left, 30)
        .set_label_area_size(LabelAreaPosition::Right, 30)
        .set_label_area_size(LabelAreaPosition::Bottom, 30)
        .caption("all Std value", ("sans-serif", 20))
        .build_cartesian_2d(0..(all_std.len() - 1), 0..(all_std_clone[(pos.trunc() as usize)] as i32 + 1))
        .unwrap();
    upper_chart.configure_mesh().draw().unwrap();
    
    upper_chart.draw_series((0..).zip(all_std.iter()).map(|(x, y)| {
        let mut bar = Rectangle::new([(x, 0), (x, *y as i32)], RED.filled());
        bar.set_margin(0, 0, 5, 5);
        bar
    }))
    .unwrap();
}
