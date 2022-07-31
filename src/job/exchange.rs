use money2::{Currency, Exchange, ExchangeRates};

use super::Job;

impl Exchange for Job
{
	type Output = Self;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		Self::Output {
			invoice: self.invoice.exchange(currency, rates),
			..self
		}
	}
}

impl Exchange for &Job
{
	type Output = Job;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		Self::Output {
			client: self.client.clone(),
			date_close: self.date_close,
			date_open: self.date_open,
			id: self.id,
			increment: self.increment,
			invoice: self.invoice.exchange(currency, rates),
			notes: self.notes.clone(),
			objectives: self.objectives.clone(),
		}
	}
}
