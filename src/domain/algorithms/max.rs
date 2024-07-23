#[derive(Debug)]
pub struct MaxAlg {
    // number of occurences of max value
    seen: usize,
    max: f64,
}

impl MaxAlg {
    pub fn new(data: impl Iterator<Item = f64>) -> Self {
        let (max, seen) = fold_max(data, f64::MIN, 0);
        Self { max, seen }
    }

    pub fn get(&self) -> f64 {
        self.max
    }

    pub fn update(
        &mut self,
        incoming: impl Iterator<Item = f64> + Clone,
        outgoing: impl Iterator<Item = f64>,
        inside: impl Iterator<Item = f64>,
    ) {
        self.seen -= outgoing.filter(|val| val == &self.max).count();
        let (max, seen) = fold_max(incoming.clone(), self.max, self.seen);

        // All current max values left window and bigger value was not found.
        // We have to look for a new one in full data window
        if seen == 0 {
            let (max, seen) = fold_max(incoming.chain(inside), f64::MIN, 0);
            self.seen = seen;
            self.max = max;
        } else {
            self.seen = seen;
            self.max = max;
        }
    }
}

fn fold_max(data: impl Iterator<Item = f64>, max: f64, seen: usize) -> (f64, usize) {
    data.fold((max, seen), |(max, seen), val| {
        if val > max {
            (val, 1)
        } else if val == max {
            (max, seen + 1)
        } else {
            (max, seen)
        }
    })
}
