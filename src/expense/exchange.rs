use money2::{Currency, Exchange, ExchangeRates};

use super::Expense;

impl Exchange for Expense
{
	fn exchange_mut(&mut self, currency: Currency, rates: &ExchangeRates)
	{
		self.cost.exchange_mut(currency, rates);
	}
}
