use super::RestoreResult;

/// Implementors of this trait are enabled to preserve data which is important (e.g. a reference
/// number), by [skipping serialization](https://serde.rs/attr-skip-serializing.html) and then using
/// `#[serde(default)]` to generate a temporary value on [deserialization](serde::Deserialize).
///
/// Then, this trait can be used to [restore](RestorableSerde::try_restore) the important data.
pub trait RestorableSerde
{
	/// Take some aspects of an `original` and restore them from [`Default`]s which were assigned
	/// upon [deserialization](serde::Deserialize).
	///
	/// Will return a [`RestoreError`](super::RestoreError) if the `original` cannot be merged with
	/// this value.
	///
	/// # Example
	///
	/// ```rust
	/// use winvoice_schema::{Employee, Id, RestorableSerde};
	/// # use pretty_assertions::{assert_eq, assert_ne};
	///
	/// let original = Employee {
	///   active: true,
	///   department: "Executive".into(),
	///   id: Id::new_v4(), // NOTE: you normally want to avoid assigning an arbitrary ID like this
	///   name: "Bob".into(),
	///   title: "CEO".into(),
	/// };
	///
	/// // Pretend this is deserialized user input…
	/// let mut edited = Employee {
	///   id: Id::new_v4(),
	///   name: "Bob Buildman".into(),
	///   ..original.clone()
	/// };
	///
	/// assert_eq!(edited.active, original.active);
	/// assert_eq!(edited.department, original.department);
	/// assert_eq!(edited.title, original.title);
	/// assert_ne!(edited.id, original.id);
	/// assert_ne!(edited.name, original.name);
	///
	/// edited.try_restore(&original).unwrap();
	///
	/// assert_eq!(edited.active, original.active);
	/// assert_eq!(edited.department, original.department);
	/// assert_eq!(edited.id, original.id);
	/// assert_eq!(edited.title, original.title);
	/// assert_ne!(edited.name, original.name);
	/// ```
	fn try_restore(&mut self, original: &Self) -> RestoreResult<()>;
}

impl<T> RestorableSerde for Vec<T>
where
	T: RestorableSerde,
{
	fn try_restore(&mut self, original: &Self) -> RestoreResult<()>
	{
		self.iter_mut()
			.zip(original)
			.try_for_each(|(edited, original)| edited.try_restore(original))
	}
}
