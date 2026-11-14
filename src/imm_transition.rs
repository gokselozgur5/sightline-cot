/// Error returned when transition matrix probability update is invalid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionUpdateError {
    InvalidMatrixDimensions,
    InvalidProbability,
    RowSumNotOne,
    NonFinite,
}
