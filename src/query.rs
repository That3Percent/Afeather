pub use crate::*;
use std::marker::PhantomData;

pub trait Query {
    type Reads: ReadableStorage;
    type Output;

    fn execute(&self, data: QueryData<Self::Reads>) -> Self::Output;
}

pub struct QueryData<'a, T> {
    i: usize,
    archetypes: &'a [Option<Archetype>],
    globals: &'a Components,
    _marker: PhantomData<&'a [T]>,
}

impl<'a, T> QueryData<'a, T> {
    pub fn new(globals: &'a Components, archetypes: &'a [Option<Archetype>]) -> Self {
        Self {
            i: 0,
            archetypes,
            globals,
            _marker: PhantomData,
        }
    }
}

impl<'a, RL: RefLike<Borrowed = B>, T: ReadableStorage<Read = RL>, B: BorrowedStorage> Iterator
    for QueryData<'a, T>
{
    type Item = B::Batch;
    fn next(&mut self) -> Option<Self::Item> {
        //self.borrow = None;
        //self.storage = None;
        while self.i < self.archetypes.len() {
            let i = self.i;
            self.i += 1;
            if let Some(candidate) = &self.archetypes[i] {
                let storage = T::get(self.globals, &candidate.components);
                if let Some(storage) = storage {
                    //self.storage = Some(storage);
                    let borrow = storage.borrow();
                    //self.borrow = Some(borrow);
                    let batch = borrow.read_batch();
                    return Some(batch);
                }
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.archetypes.len() - self.i))
    }
}
