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
    //create vec<Feature> from dataset file and init value
    pub fn new_and_init(contents: String) -> Result<Vec<Feature>, String> {
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
            x.min = x.calc_min();
            x.p25 = x.calc_percentile(25.0);
            x.p50 = x.calc_percentile(50.0);
            x.p75 = x.calc_percentile(75.0);
            x.max = x.calc_max();
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

    fn calc_min(&self) -> Option<f32> {
        let new_values: Result<Vec<f32>, _> = self.values.iter().map(|x| x.parse()).collect();
        match new_values {
            Ok(all_values) => {
                let mut ret = match all_values.get(1) {
                    Some(value) => value,
                    _ => return None
                };
                all_values.iter().for_each(|x| {
                    if x < ret {
                        ret = x;
                    }
                });
                Some(*ret)
            },
            Err(_) => None,
        }
    }

    fn calc_max(&self) -> Option<f32> {
        let new_values: Result<Vec<f32>, _> = self.values.iter().map(|x| x.parse()).collect();
        match new_values {
            Ok(all_values) => {
                let mut ret = match all_values.get(1) {
                    Some(value) => value,
                    _ => return None
                };
                all_values.iter().for_each(|x| {
                    if x > ret {
                        ret = x;
                    }
                });
                Some(*ret)
            },
            Err(_) => None,
        }
    }

    fn calc_percentile(&self, percentile: f32) -> Option<f32> {
        let new_values: Result<Vec<f32>, _> = self.values.iter().map(|x| x.parse()).collect();
        match new_values {
            Ok(mut all_values) => {
                all_values.sort_by(|a, b| {
                    match a.partial_cmp(b) {
                        Some(ord) => ord,
                        _ => {
                            println!("unexpected error when sort values");
                            unreachable!();
                        }
                    }
                });
                let pos: f32 = percentile / 100.0 * (all_values.len() as f32 + 1.0);
                match pos.fract() {
                    0.0 => {
                        match all_values.get(pos as usize - 1) {
                            Some(value) => Some(*value),
                            _ => None
                        }
                    }
                    _ => {
                        let n1 = all_values.get(pos.trunc() as usize - 1);
                        let n2 = all_values.get(pos.trunc() as usize);
                        match n1 {
                            Some(nb1) => {
                                match n2 {
                                    Some(nb2) => {
                                        Some((*nb1 + *nb2) / 2.0)
                                    },
                                    _ => None
                                }
                            },
                            _ => None
                        }
                    }
                }
            },
            Err(_) => None,
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

    pub fn get_min(&self) -> Option<f32> {
        self.min
    }

    pub fn get_max(&self) -> Option<f32> {
        self.max
    }

    pub fn get_p25(&self) -> Option<f32> {
        self.p25
    }

    pub fn get_p50(&self) -> Option<f32> {
        self.p50
    }

    pub fn get_p75(&self) -> Option<f32> {
        self.p75
    }
}

