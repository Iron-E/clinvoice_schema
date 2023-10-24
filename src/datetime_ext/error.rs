use chrono::{OutOfRangeError, RoundingError};
use thiserror::Error;

/// An error resulting from [incrementing](super::DateTimeExt::increment_with) a [`DateTime`](chrono::DateTime).
#[derive(Debug, Error)]
pub enum IncrementError
{
	#[allow(missing_docs)]
	#[error(transparent)]
	OutOfRange(#[from] OutOfRangeError),

	#[allow(missing_docs)]
	#[error(transparent)]
	Rounding(#[from] RoundingError),
}

/// A [`Result`] with an [`IncrementError`].
pub type IncrementResult<T> = Result<T, IncrementError>;
