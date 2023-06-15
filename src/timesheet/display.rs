use core::fmt::{Display, Formatter, Result};

use chrono::{DateTime, Local};

use super::Timesheet;

impl Display for Timesheet
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		/// One indent in, with a newline.
		const NEWLINE_INDENT: &str = "\n\t";

		/// Two indents in, with a newline.
		const NEWLINE_TWO_INDENTS: &str = "\n\t\t";

		writeln!(
			formatter,
			"{} – {}",
			DateTime::<Local>::from(self.time_begin).naive_local(),
			self.time_end.map_or_else(
				|| "Current".into(),
				|time| DateTime::<Local>::from(time).naive_local().to_string(),
			),
		)?;

		write!(formatter, "\t- Employee: {} {}", self.employee.title, self.employee.name)?;

		if !self.expenses.is_empty()
		{
			write!(formatter, "{NEWLINE_INDENT}- Expenses:")?;
			self.expenses.iter().try_for_each(|e| {
				write!(
					formatter,
					"{NEWLINE_TWO_INDENTS}{}",
					e.to_string().replace('\n', NEWLINE_TWO_INDENTS)
				)
			})?;
		}

		if !self.work_notes.is_empty()
		{
			write!(
				formatter,
				"{NEWLINE_INDENT}- Work Notes:{NEWLINE_TWO_INDENTS}{}",
				self.work_notes.replace('\n', NEWLINE_TWO_INDENTS)
			)?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	use core::time::Duration;

	use chrono::Utc;
	use money2::{Currency, Money};
	use pretty_assertions::assert_eq;

	use super::{DateTime, Local, Timesheet};
	use crate::{Employee, Expense, Id, Invoice, Job, Location, Organization};

	#[test]
	fn display()
	{
		let earth_view = Location { name: "Earth".into(), ..Default::default() };

		let usa_view =
			Location { name: "USA".into(), outer: Some(earth_view.into()), ..Default::default() };

		let arizona_view =
			Location { name: "Arizona".into(), outer: Some(usa_view.into()), ..Default::default() };

		let phoenix_view = Location {
			name: "Phoenix".into(),
			outer: Some(arizona_view.into()),
			..Default::default()
		};

		let street_view = Location {
			name: "1337 Some Street".into(),
			outer: Some(phoenix_view.into()),
			..Default::default()
		};

		let timesheet = Timesheet {
			employee: Employee {
				name: "Testy McTesterson".into(),
				status: "Representative".into(),
				title: "CEO of Tests".into(),
				..Default::default()
			},
			expenses: vec![
				Expense {
					id: Id::new_v4(),
					category: "Food".into(),
					cost: Money::new(20_50, 2, Currency::Usd),
					description: "Fast Food™".into(),
					..Default::default()
				},
				Expense {
					id: Id::new_v4(),
					category: "Travel".into(),
					cost: Money::new(10_00, 2, Currency::Usd),
					description: "Gas".into(),
					..Default::default()
				},
			],
			job: Job {
				client: Organization {
					location: street_view,
					name: "Big Test Organization".into(),
					..Default::default()
				},
				increment: Duration::new(900, 0),
				invoice: Invoice {
					hourly_rate: Money::new(13_00, 2, Currency::Usd),
					..Default::default()
				},
				..Default::default()
			},
			time_end: Some(Utc::today().and_hms(23, 59, 59)),
			work_notes: "Went to non-corporate fast food restaurant for business meeting".into(),
			..Default::default()
		};

		assert_eq!(
			timesheet.to_string(),
			format!(
				"{} – {}
	- Employee: CEO of Tests Testy McTesterson
	- Expenses:
		№{} – Food (20.50 USD)
			Fast Food™
		№{} – Travel (10.00 USD)
			Gas
	- Work Notes:
		Went to non-corporate fast food restaurant for business meeting",
				DateTime::<Local>::from(timesheet.time_begin).naive_local(),
				DateTime::<Local>::from(timesheet.time_end.unwrap()).naive_local(),
				timesheet.expenses.get(0).unwrap().id,
				timesheet.expenses.get(1).unwrap().id,
			),
		);
	}
}
