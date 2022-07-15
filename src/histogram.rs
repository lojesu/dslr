use plotters::prelude::*;
use crate::feature::Feature;

pub fn histogram(feature: Feature, all_rsd: Vec<f32>) {
    let root = BitMapBackend::new("graphics/histogram.png", (640, 480)).into_drawing_area();
        root.fill(&WHITE).unwrap();

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
        bar.set_margin(0, 0, 5, 5);
        bar
    }))
    .unwrap();
}
