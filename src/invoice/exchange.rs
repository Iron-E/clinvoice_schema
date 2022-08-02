use money2::{Currency, Exchange, ExchangeRates};

use super::Invoice;

impl Exchange for Invoice
{
	fn exchange_mut(&mut self, currency: Currency, rates: &ExchangeRates)
	{
		self.hourly_rate = self.hourly_rate.exchange(currency, rates);
	}
}
