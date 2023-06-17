use core::cmp::Ordering;

use super::Job;

impl PartialOrd for Job
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		match self.client.cmp(&other.client).then_with(|| {
			self.date_close.cmp(&other.date_close).then_with(|| {
				self.date_open.cmp(&other.date_open).then_with(|| {
					self.id.cmp(&other.id).then_with(|| {
						self.increment.cmp(&other.increment).then_with(|| {
							self.notes
								.cmp(&other.notes)
								.then_with(|| self.objectives.cmp(&other.objectives))
						})
					})
				})
			})
		})
		{
			Ordering::Equal => None,
			o => Some(o),
		}
	}
}
