use crate::error::Result;
use bc_envelope::prelude::*;

pub trait Indexed {
    fn index(&self) -> usize;
    fn set_index(&mut self, index: usize);
}

pub fn set_indexes<T: Indexed>(mut vec: Vec<T>) -> Vec<T> {
    for (index, item) in vec.iter_mut().enumerate() {
        item.set_index(index);
    }
    vec
}

pub trait SetIndexes<T> {
    fn set_indexes(self) -> Self;
}

impl<T: Indexed> SetIndexes<T> for Vec<T> {
    fn set_indexes(self) -> Self {
        set_indexes(self)
    }
}

impl<T: Indexed> SetIndexes<T> for Option<Vec<T>> {
    fn set_indexes(self) -> Self {
        self.map(|v| set_indexes(v))
    }
}

pub fn envelope_optional_indexed_objects_for_predicate<T>(
    envelope: &Envelope,
    predicate: impl AsRef<str>,
) -> Result<Option<Vec<T>>>
where
    T: Indexed + TryFrom<Envelope, Error = bc_envelope::Error> + 'static,
{
    let mut vec = envelope
        .try_objects_for_predicate::<T>(predicate.as_ref())
        .map_err(crate::error::Error::from)?;
    vec.sort_by_key(|input| input.index());
    Ok((!vec.is_empty()).then_some(vec))
}

pub fn envelope_indexed_objects_for_predicate<T>(
    envelope: &Envelope,
    predicate: impl AsRef<str>,
) -> Result<Vec<T>>
where
    T: Indexed + TryFrom<Envelope, Error = bc_envelope::Error> + 'static,
{
    let mut vec = envelope
        .try_objects_for_predicate::<T>(predicate.as_ref())
        .map_err(crate::error::Error::from)?;
    vec.sort_by_key(|input| input.index());
    Ok(vec)
}
