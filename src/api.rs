use crate::domain::computation::State;

pub mod add_batch;
pub mod stats;

#[derive(Debug)]
pub struct AppState {
    pub state: State,
}
