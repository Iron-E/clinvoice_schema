use money2::{Currency, Exchange, ExchangeRates};

use super::Timesheet;

impl Exchange for Timesheet
{
	fn exchange_mut(&mut self, currency: Currency, rates: &ExchangeRates)
	{
		self.expenses.exchange_mut(currency, rates);
		self.job.exchange_mut(currency, rates);
	}
}
