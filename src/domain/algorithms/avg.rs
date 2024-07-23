// Rolling average algorithm
#[derive(Debug)]
pub struct AverageAlg {
    average: f64,
    window_size: usize,
}

impl AverageAlg {
    pub fn new(data: impl Iterator<Item = f64> + Clone) -> Self {
        let window_size = data.clone().count();
        let average = data.fold(0f64, |avg, value| avg + value) / window_size as f64;

        Self {
            window_size,
            average,
        }
    }

    pub fn get(&self) -> f64 {
        self.average
    }

    // Apply incremental change for fast update
    pub fn update(
        &mut self,
        incoming: impl Iterator<Item = f64>,
        outgoing: impl Iterator<Item = f64>,
    ) {
        self.average = incoming
            .zip(outgoing)
            .fold(self.average, |avg, (incoming, outgoing)| {
                avg + (incoming - outgoing) / (self.window_size as f64)
            });
    }
}
