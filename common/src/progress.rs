#[derive(Debug, Clone, Copy)]
pub enum ProgressState {
    Pending,
    InProgress,
    Finished,
}

#[derive(Debug, Clone, Copy)]
pub struct ProgressUpdate {
    /// holds the percentage between zero and one,
    /// is NaN when the percentage is not (yet) determinable
    pub percentage: f64,
    pub state: ProgressState,
}
