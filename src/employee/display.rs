use core::fmt::{Display, Formatter, Result};

use super::Employee;

impl Display for Employee
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(
			formatter,
			"{}{} {} of the {} department",
			match self.active
			{
				false => "Ex-",
				true => "",
			},
			self.title,
			self.name,
			self.department,
		)
	}
}

#[cfg(test)]
mod tests
{
	use pretty_assertions::assert_str_eq;

	use super::Employee;
	use crate::Id;

	#[test]
	fn display()
	{
		let mut employee = Employee {
			active: true,
			department: "Executive".into(),
			id: Id::default(),
			name: "Testy McTesterson".into(),
			title: "Chief Test Officer".into(),
		};

		assert_str_eq!(
			employee.to_string(),
			"Chief Test Officer Testy McTesterson of the Executive department",
		);

		employee.active = false;
		assert_str_eq!(
			employee.to_string(),
			"Ex-Chief Test Officer Testy McTesterson of the Executive department",
		);
	}
}
