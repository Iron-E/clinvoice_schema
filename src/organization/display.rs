use core::fmt::{Display, Formatter, Result};

use super::Organization;

impl Display for Organization
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(formatter, "{} @ {}", self.name, self.location)?;

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	use pretty_assertions::assert_str_eq;

	use super::Organization;
	use crate::Location;

	#[test]
	fn display()
	{
		let organization = Organization {
			location: Location {
				name: "Arizona".into(),
				outer: Some(
					Location {
						name: "USA".into(),
						outer: Some(Location { name: "Earth".into(), ..Default::default() }.into()),
						..Default::default()
					}
					.into(),
				),
				..Default::default()
			},
			name: "Big Old Test".into(),
			..Default::default()
		};

		assert_str_eq!(organization.to_string(), "Big Old Test @ Arizona, USA, Earth");
	}
}
