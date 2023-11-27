use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Statistics {
    max: f64,
    min: f64,
    geometric_mean: f64,
    algebraic_mean: f64,
    variance: f64,
    standard_deviation: f64,
    /// 第一四分位点
    q1: f64,
    /// 第三四分位点
    q3: f64,
}

impl Statistics {
    fn new() -> Self {
        Statistics {
            max: 0.0,
            min: 0.0,
            geometric_mean: 0.0,
            algebraic_mean: 0.0,
            variance: 0.0,
            standard_deviation: 0.0,
            q1: 0.0,
            q3: 0.0,
        }
    }

    pub fn from(mut data: Vec<f64>) -> Self {
        if data.len() == 0 {
            return Self::new();
        }

        let max;
        let min;
        let mut geometric_mean = 1.0;
        let mut algebraic_mean = 0.0;
        let mut variance = 0.0;
        let standard_deviation;

        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        max = data.last().unwrap().clone();
        min = data.first().unwrap().clone();

        //G_n=sqrt(PI(xi), n);
        data.iter()
            .for_each(|x| geometric_mean *= x.powf(1.0 / data.len() as f64));
        // geometric_mean = geometric_mean.powf(1.0/data.len() as f64);

        //A_n=Sum(xi)/n;
        data.iter()
            .for_each(|x| algebraic_mean += x / data.len() as f64);
        // algebraic_mean /= data.len() as f64;

        //V=(Sum(xi-A)^2)/n
        data.iter()
            .for_each(|x| variance += (x - algebraic_mean).powi(2));
        variance /= data.len() as f64;

        //Sd=sqrt(V)
        standard_deviation = variance.sqrt();

        Statistics {
            max,
            min,
            geometric_mean,
            algebraic_mean,
            variance,
            standard_deviation,
            q1: quartile(&data, 0.25),
            q3: quartile(&data, 0.75),
        }
    }
}

/// calculate quartile_1 and quartile_3
fn quartile(data: &Vec<f64>, percentile: f64) -> f64 {
    let n = data.len();
    let index = (percentile * (n - 1) as f64) as usize;
    if n == 1 {
        data[0]
    } else if index == n - 1 {
        data[n - 1]
    } else {
        let lower = data[index];
        let upper = data[index + 1];
        lower + (upper - lower) * (percentile * (n - 1) as f64 - index as f64)
    }
}
