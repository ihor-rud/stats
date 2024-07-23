#[derive(Debug)]
pub struct VarianceAlg {
    variance: f64,
    average: f64,
    window_size: usize,
}

// Rolling variance algorithm
impl VarianceAlg {
    pub fn new(data: impl Iterator<Item = f64> + Clone) -> Self {
        let window_size = data.clone().count();
        let average = data.clone().fold(0f64, |sum, value| sum + value) / window_size as f64;
        let variance = data.fold(0f64, |sum, value| sum + (value - average).powi(2))
            / (window_size - 1) as f64;

        Self {
            window_size,
            average,
            variance,
        }
    }

    pub fn get(&self) -> f64 {
        self.variance
    }

    // Apply incremental change for fast update
    pub fn update(
        &mut self,
        incoming: impl Iterator<Item = f64> + Clone,
        outgoing: impl Iterator<Item = f64> + Clone,
    ) {
        let (new_avg, new_var) = incoming.zip(outgoing).fold(
            (self.average, self.variance),
            |(avg, var), (incoming, outgoing)| {
                let new_avg = avg + (incoming - outgoing) / (self.window_size as f64);
                let new_var = var
                    + (incoming - outgoing) * (incoming - new_avg + outgoing - avg)
                        / ((self.window_size - 1) as f64);
                (new_avg, new_var)
            },
        );

        self.average = new_avg;
        self.variance = new_var;
    }
}
