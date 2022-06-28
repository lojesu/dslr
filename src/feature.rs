/*
name => name of this feature
values => all values of this feature
count => number of value(Non-Null) for this feature
#If it makes sense
mean => average value
std => standard deviation of values
min => minimum value
25% => percentile value in our ordonate list
50% => percentile value in our ordonate list
75% => percentile value in our ordonate list
max => maximum value
# If it makes sense
unique => number of unique value
top => value the most represented
freq => frequency of top value
*/

#[derive(Debug)]
pub struct Feature {
    name: String,
    values: Vec<String>,
    count: usize,
    mean: Option<f32>,
    std: Option<f32>,
    min: Option<f32>,
    p25: Option<f32>,
    p50: Option<f32>,
    p75: Option<f32>,
    max: Option<f32>,
    unique: Option<i32>,
    top: Option<String>,
    freq: Option<i32>,
}

impl Feature {
    //create vec<Feature> from dataset file
    pub fn new(contents: String) -> Result<Vec<Feature>, String> {
        let mut ret: Vec<Feature> = Vec::default();
        let mut lines = contents.split('\n');
        match lines.next() {
            Some(first_line) => {
                first_line.split(',').for_each(|x| {
                    ret.push(Feature {
                        name: x.to_string(),
                        values: Vec::default(),
                        count: 0,
                        mean: None,
                        std: None,
                        min: None,
                        p25: None,
                        p50: None,
                        p75: None,
                        max: None,
                        unique: None,
                        top: None,
                        freq: None
                    });
                });
            },
            _ => return Err("no line in dataset".to_string())
        }
        let mut i = 0;
        let nb_lines = lines.clone().collect::<Vec<&str>>().len();
        while i < nb_lines - 1 {
            match lines.next() {
                Some(line) => {
                    for (j, r) in ret.iter_mut().enumerate() {
                        let value_to_push: Vec<&str> = line.split(',').collect();
                        if value_to_push[j].len() > 0 {
                            r.values.push(value_to_push[j].to_string());
                        }
                        if i == nb_lines - 2 {
                            r.count = r.values.len();
                        }
                    }
                },
                _ => return Err("unexpected error in dataset".to_string()),
            }
            i += 1;
        }
        ret.iter_mut().for_each(|x| {
            x.mean = x.calc_mean();
            x.std = x.calc_std();
        });
        Ok(ret)
    }

    // all calculate function
    fn calc_mean(&self) -> Option<f32> {
        let new_values: Result<Vec<f32>, _> = self.values.iter().map(|x| x.parse()).collect();
        match new_values {
            Ok(all_values) => {
                let len = all_values.len();
                if len == 0 {
                    return None
                }
                let sum: f32 = all_values.iter().sum();
                Some(sum / (len as f32))
            },
            Err(_) => None
        }
    }

    fn calc_std(&self) -> Option<f32> {
        let new_values: Result<Vec<f32>, _> = self.values.iter().map(|x| x.parse()).collect();
        match new_values {
            Ok(all_values) => {
                let len = all_values.len();
                if len == 0 {
                    return None
                }
                let all_gap_pow_add: f32 = all_values.iter().map(|x| {
                    let gap = match self.get_mean() {
                        Some(mean) => x - mean,
                        _ => unreachable!(),
                    };
                    gap.powf(2.0)
                }).sum();
                Some((all_gap_pow_add / (len as f32)).sqrt())
            }
            Err(_) => None
        }
    }

    //all get function
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_count(&self) -> usize {
        self.count
    }

    pub fn get_mean(&self) -> Option<f32> {
        self.mean
    }

    pub fn get_std(&self) -> Option<f32> {
        self.std
    }
}

