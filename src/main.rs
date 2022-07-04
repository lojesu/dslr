mod describe;
mod print;
mod feature;
mod graph;

use describe::describe;
use graph::graph;

fn main() {
    let features = describe();
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
    //graph(feature_std_min.clone(), all_std);
    graph(features[8].clone(), all_std);
}
