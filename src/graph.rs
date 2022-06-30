use plotters::prelude::*;
use crate::feature::Feature;

pub fn graph(feature: Feature) {
    let root_area = BitMapBackend::new("histogram.png", (900, 400))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d((0..feature.get_count()).into_segmented(), (feature.get_min().unwrap() as i32)..(feature.get_max().unwrap() as i32 + 1))
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

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
            ctx.draw_series((0..).zip(data.iter()).map(|(x, y)| {
                let x0 = SegmentValue::Exact(x);
                let x1 = SegmentValue::Exact(x + 1);
                let mut bar = Rectangle::new([(x0, 0), (x1, *y as i32)], RED.filled());
                bar.set_margin(0, 0, 2, 2);
                bar
            }))
            .unwrap();
            },
        _ => ()
    }
}
