use money2::{Currency, Exchange, ExchangeRates};

use super::Expense;

impl Exchange for Expense
{
	type Output = Self;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		Self::Output {
			cost: self.cost.exchange(currency, rates),
			..self
		}
	}
}

impl Exchange for &Expense
{
	type Output = Expense;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		Self::Output {
			category: self.category.clone(),
			cost: self.cost.exchange(currency, rates),
			description: self.description.clone(),
			id: self.id,
			timesheet_id: self.timesheet_id,
		}
	}
}
