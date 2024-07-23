use circular_buffer::CircularBuffer;
use dashmap::DashMap;

use super::algorithms::avg::AverageAlg;
use super::algorithms::last::LastAlg;
use super::algorithms::max::MaxAlg;
use super::algorithms::min::MinAlg;
use super::algorithms::var::VarianceAlg;
use super::models::{Stats, StatsError, WindowSize};

#[derive(Debug)]
struct Computations {
    avg: AverageAlg,
    last: LastAlg,
    max: MaxAlg,
    min: MinAlg,
    var: VarianceAlg,
}

impl Computations {
    fn new(data: impl Iterator<Item = f64> + Clone) -> Self {
        Self {
            avg: AverageAlg::new(data.clone()),
            last: LastAlg::new(data.clone()),
            max: MaxAlg::new(data.clone()),
            min: MinAlg::new(data.clone()),
            var: VarianceAlg::new(data),
        }
    }

    fn update(
        &mut self,
        incoming: impl Iterator<Item = f64> + Clone,
        outgoing: impl Iterator<Item = f64> + Clone,
        inside: impl Iterator<Item = f64> + Clone,
    ) {
        self.avg.update(incoming.clone(), outgoing.clone());
        self.last.update(incoming.clone());
        self.max
            .update(incoming.clone(), outgoing.clone(), inside.clone());
        self.min
            .update(incoming.clone(), outgoing.clone(), inside.clone());
        self.var.update(incoming, outgoing);
    }
}

// contains computations for different window sizes (10, 100, 1000 etc)
#[derive(Debug, Default)]
struct WindowedComputations {
    e1: Option<Computations>,
    e2: Option<Computations>,
    e3: Option<Computations>,
    e4: Option<Computations>,
    e5: Option<Computations>,
    e6: Option<Computations>,
    e7: Option<Computations>,
    e8: Option<Computations>,
}

impl WindowedComputations {
    fn get_window(&self, size: WindowSize) -> Option<&Computations> {
        match size {
            WindowSize::E1 => self.e1.as_ref(),
            WindowSize::E2 => self.e2.as_ref(),
            WindowSize::E3 => self.e3.as_ref(),
            WindowSize::E4 => self.e4.as_ref(),
            WindowSize::E5 => self.e5.as_ref(),
            WindowSize::E6 => self.e6.as_ref(),
            WindowSize::E7 => self.e7.as_ref(),
            WindowSize::E8 => self.e8.as_ref(),
        }
    }

    fn get_window_mut(&mut self, size: WindowSize) -> Option<&mut Computations> {
        match size {
            WindowSize::E1 => self.e1.as_mut(),
            WindowSize::E2 => self.e2.as_mut(),
            WindowSize::E3 => self.e3.as_mut(),
            WindowSize::E4 => self.e4.as_mut(),
            WindowSize::E5 => self.e5.as_mut(),
            WindowSize::E6 => self.e6.as_mut(),
            WindowSize::E7 => self.e7.as_mut(),
            WindowSize::E8 => self.e8.as_mut(),
        }
    }

    fn insert(&mut self, size: WindowSize, computation: Computations) {
        match size {
            WindowSize::E1 => self.e1 = Some(computation),
            WindowSize::E2 => self.e2 = Some(computation),
            WindowSize::E3 => self.e3 = Some(computation),
            WindowSize::E4 => self.e4 = Some(computation),
            WindowSize::E5 => self.e5 = Some(computation),
            WindowSize::E6 => self.e6 = Some(computation),
            WindowSize::E7 => self.e7 = Some(computation),
            WindowSize::E8 => self.e8 = Some(computation),
        }
    }
}

#[derive(Debug, Default)]
pub struct State {
    computations: DashMap<String, WindowedComputations>,
    storages: DashMap<String, Box<CircularBuffer<100_000_000, f64>>>, // data order is from oldest to newest
}

impl State {
    pub fn get_stats(&self, name: &str, size: WindowSize) -> Result<Stats, StatsError> {
        let computations = self
            .computations
            .get(name)
            .ok_or(StatsError::NameNotFound)?;
        let computation = computations
            .get_window(size)
            .ok_or(StatsError::WindowIsNotFilled)?;

        Ok(Stats {
            avg: computation.avg.get(),
            min: computation.min.get(),
            max: computation.max.get(),
            last: computation.last.get(),
            var: computation.var.get(),
        })
    }

    pub fn add_batch(&self, name: String, batch: Vec<f64>) {
        let mut computations = self.computations.entry(name.clone()).or_default();
        let mut storage = self
            .storages
            .entry(name)
            .or_insert_with(CircularBuffer::boxed);

        for size in [
            WindowSize::E1,
            WindowSize::E2,
            WindowSize::E3,
            WindowSize::E4,
            WindowSize::E5,
            WindowSize::E6,
            WindowSize::E7,
            WindowSize::E8,
        ] {
            let windows_size = size.into();

            if windows_size > (storage.len() + batch.len()) {
                break;
            }

            // its faster to use non incremental version of update if incoming data size is bigger then window size
            if batch.len() >= windows_size {
                let window = batch.iter().rev().take(windows_size).rev().cloned();
                computations.insert(size, Computations::new(window));
            } else {
                let computation = computations.get_window_mut(size);

                match computation {
                    // preform incremental update
                    Some(computation) => {
                        let incoming = batch.iter().cloned();
                        let outgoing = storage
                            .iter()
                            .rev()
                            .take(windows_size)
                            .rev()
                            .take(batch.len())
                            .cloned();
                        let inside = storage
                            .iter()
                            .rev()
                            .take(windows_size - batch.len())
                            .rev()
                            .cloned();
                        computation.update(incoming, outgoing, inside);
                    }
                    // we just got enough data to initialize window
                    None => {
                        let window = storage
                            .iter()
                            .rev()
                            .take(windows_size - batch.len())
                            .rev()
                            .chain(batch.iter())
                            .cloned();
                        computations.insert(size, Computations::new(window));
                    }
                }
            }
        }

        storage.extend(batch);
    }
}
