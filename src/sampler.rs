pub struct Sampler {
    trial: u32,
    mean: f64,
}

impl Sampler {
    pub fn new() -> Self {
        Sampler {
            trial: 0_u32,
            mean: 0_f64,
        }
    }

    /// Update the empirical mean by the observed value.
    pub fn observe(&mut self, val: f64) -> &mut Self {
        self.mean = (self.mean * (self.trial as f64) + val) / ((self.trial + 1) as f64);
        self.trial += 1;
        self
    }

    /// Return the empirical mean.
    pub fn get_mean(&self) -> f64 {
        self.mean
    }

    /// Return the number of observations.
    pub fn get_trial(&self) -> u32 {
        self.trial
    }
}
