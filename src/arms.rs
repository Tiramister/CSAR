use rand::prelude::ThreadRng;
use rand_distr::{Distribution, Normal};

pub struct Arm {
    distr: Normal<f64>,
    rng: ThreadRng,
}

impl Arm {
    pub fn new(mean: f64, std_dev: f64) -> Self {
        Arm {
            distr: Normal::new(mean, std_dev).unwrap(),
            rng: rand::thread_rng(),
        }
    }
    pub fn sample(&mut self) -> f64 {
        self.distr.sample(&mut self.rng)
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

    pub fn sample(&mut self, i: usize) -> f64 {
        self.arms[i].sample()
    }

    pub fn get_mean(&mut self, i: usize) -> f64 {
        self.arms[i].sample()
    }
}

pub type Weights = Vec<f64>;
