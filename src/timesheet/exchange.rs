use money2::{Currency, Exchange, ExchangeRates};

use super::Timesheet;

impl Exchange for Timesheet
{
	type Output = Self;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		Self::Output {
			expenses: self.expenses.exchange(currency, rates),
			job: self.job.exchange(currency, rates),
			..self
		}
	}
}

impl Exchange for &Timesheet
{
	type Output = Timesheet;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		Self::Output {
			employee: self.employee.clone(),
			expenses: self.expenses.as_slice().exchange(currency, rates),
			id: self.id,
			job: (&self.job).exchange(currency, rates),
			time_begin: self.time_begin,
			time_end: self.time_end,
			work_notes: self.work_notes.clone(),
		}
	}
}
