mod error;

use core::time::Duration as StdDuration;

use chrono::{DateTime, Duration, DurationRound, TimeZone};
pub use error::{IncrementError, IncrementResult};

/// Extension methods for [`DateTime`].
pub trait DateTimeExt
where
	Self: Sized,
{
	/// Increment some start date, and an `end` date with a given [`Duration`].
	fn increment_with(self, end: Self, increment: StdDuration) -> IncrementResult<(Self, Self)>;
}

impl<Tz> DateTimeExt for DateTime<Tz>
where
	DateTime<Tz>: Copy + core::fmt::Display,
	Tz: TimeZone,
{
	fn increment_with(self, end: Self, increment: StdDuration) -> IncrementResult<(Self, Self)>
	{
		if increment.is_zero()
		{
			return Ok((self, end));
		}

		let duration = Duration::from_std(increment)?;
		let start = self.duration_round(duration)?;
		match end.duration_round(duration)
		{
			Ok(d) if d == start => Ok((start, start + duration)),
			Ok(d) => Ok((start, d)),
			Err(e) => Err(e.into()),
		}
	}
}
