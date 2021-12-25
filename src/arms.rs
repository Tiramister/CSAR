use rand_distr::{Distribution, Normal};

pub struct Arm {
    distr: Normal<f64>,
}

impl Arm {
    pub fn new(mean: f64, std_dev: f64) -> Self {
        Arm {
            distr: Normal::new(mean, std_dev).unwrap(),
        }
    }
    pub fn sample(&self) -> f64 {
        self.distr.sample(&mut rand::thread_rng())
    }

    pub fn get_mean(&self) -> f64 {
        self.distr.mean()
    }
}

pub struct Arms {
    arms: Vec<Arm>,
}

impl Arms {
    pub fn new() -> Self {
        Arms { arms: Vec::new() }
    }

    pub fn add_arm(&mut self, mean: f64, std_dev: f64) {
        self.arms.push(Arm::new(mean, std_dev))
    }

    pub fn sample(&self, i: usize) -> f64 {
        self.arms[i].sample()
    }

    pub fn get_mean(&self, i: usize) -> f64 {
        self.arms[i].sample()
    }
}

pub type Weights = Vec<f64>;
