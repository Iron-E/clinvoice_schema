use money2::{Currency, Exchange, ExchangeRates};

use super::Invoice;

impl Exchange for Invoice
{
	type Output = Self;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		Self {
			hourly_rate: self.hourly_rate.exchange(currency, rates),
			..self
		}
	}
}

impl Exchange for &Invoice
{
	type Output = Invoice;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		Self::Output {
			date: self.date,
			hourly_rate: self.hourly_rate.exchange(currency, rates),
		}
	}
}
