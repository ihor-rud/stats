#[derive(Debug)]
pub struct LastAlg {
    last: f64,
}

impl LastAlg {
    pub fn new(data: impl Iterator<Item = f64>) -> Self {
        Self {
            last: data.last().unwrap(),
        }
    }

    pub fn get(&self) -> f64 {
        self.last
    }

    pub fn update(&mut self, incoming: impl Iterator<Item = f64>) {
        let Some(new_last) = incoming.last() else {
            return;
        };

        self.last = new_last;
    }
}
