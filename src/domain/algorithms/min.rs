#[derive(Debug)]
pub struct MinAlg {
    // number of occurences of min value
    seen: usize,
    min: f64,
}

impl MinAlg {
    pub fn new(data: impl Iterator<Item = f64>) -> Self {
        let (min, seen) = fold_min(data, f64::MAX, 0);
        Self { min, seen }
    }

    pub fn get(&self) -> f64 {
        self.min
    }

    pub fn update(
        &mut self,
        incoming: impl Iterator<Item = f64> + Clone,
        outgoing: impl Iterator<Item = f64>,
        inside: impl Iterator<Item = f64>,
    ) {
        self.seen -= outgoing.filter(|val| val == &self.min).count();

        let (min, seen) = fold_min(incoming.clone(), self.min, self.seen);

        // Same as for [`MaxAlg::update`].
        if seen == 0 {
            let (min, seen) = fold_min(incoming.chain(inside), f64::MAX, 0);
            self.seen = seen;
            self.min = min;
        } else {
            self.seen = seen;
            self.min = min;
        }
    }
}

fn fold_min(data: impl Iterator<Item = f64>, min: f64, seen: usize) -> (f64, usize) {
    data.fold((min, seen), |(min, seen), val| {
        if val < min {
            (val, 1)
        } else if val == min {
            (min, seen + 1)
        } else {
            (min, seen)
        }
    })
}
