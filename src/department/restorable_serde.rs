use super::Department;
use crate::{RestorableSerde, RestoreResult};

impl RestorableSerde for Department
{
	fn try_restore(&mut self, original: &Self) -> RestoreResult<()>
	{
		self.id = original.id;
		Ok(())
	}
}
