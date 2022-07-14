use core::fmt::{Display, Formatter, Result};

use super::ContactKind;

impl Display for ContactKind
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		match self
		{
			Self::Address(location) => location.fmt(formatter),
			Self::Email(s) | Self::Other(s) | Self::Phone(s) => s.fmt(formatter),
		}
	}
}
