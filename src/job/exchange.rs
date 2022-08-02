use money2::{Currency, Exchange, ExchangeRates};

use super::Job;

impl Exchange for Job
{
	fn exchange_mut(&mut self, currency: Currency, rates: &ExchangeRates)
	{
		self.invoice.exchange_mut(currency, rates);
	}
}
