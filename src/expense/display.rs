use core::fmt::{Display, Formatter, Result};

use super::Expense;

impl Display for Expense
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		writeln!(formatter, "№{} – {} ({})", self.id, self.category, self.cost)?;
		write!(formatter, "\t{}", self.description.replace('\n', "\n\t"))
	}
}

#[cfg(test)]
mod tests
{
	use money2::{Currency, Money};
	use pretty_assertions::assert_str_eq;

	use super::Expense;
	use crate::Id;

	#[test]
	fn display()
	{
		let expense = Expense {
			id: Id::new_v4(),
			category: "Food".into(),
			cost: Money::new(20_00, 2, Currency::Usd),
			description: "Take-out for 2".into(),
			..Default::default()
		};

		assert_str_eq!(
			expense.to_string(),
			format!(
				"№{} – Food (20.00 USD)
	Take-out for 2",
				expense.id
			),
		);
	}
}
