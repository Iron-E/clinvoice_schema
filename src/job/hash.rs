use core::hash::{Hash, Hasher};

use super::Job;

impl Hash for Job
{
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.client.hash(state);
		self.date_close.hash(state);
		self.date_open.hash(state);
		self.id.hash(state);
		self.increment.hash(state);
		self.invoice.hash(state);
		self.notes.hash(state);
		self.objectives.hash(state);
	}
}
