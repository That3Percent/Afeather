pub use crate::*;
use std::marker::PhantomData;

pub trait Process {
    type Reads: ReadableStorage;
    type Writes: WritableStorage;

    fn execute(
        &self,
        read: <<<<Self as Process>::Reads as ReadableStorage>::Read as RefLike>::Borrowed as BorrowedStorage>::Batch,
        write: <<<<Self as Process>::Writes as WritableStorage>::ReadMut as RefLikeMut>::BorrowedMut as BorrowedStorageMut>::BatchMut,
    );
}

pub trait ProcessSimple {
    type Reads: ReadableStorage;
    type Writes: WritableStorage;

    fn execute(&self,
		read: <<<<Self as ProcessSimple>::Reads as ReadableStorage>::Read as RefLike>::Borrowed as BorrowedStorage>::Item)
		-> <<<<Self as ProcessSimple>::Writes as WritableStorage>::ReadMut as RefLikeMut>::BorrowedMut as BorrowedStorageMut>::ItemMut;
}

/*
pub struct ProcessFrom<TRead, TWrite> {
	_marker: PhantomData<(*const TRead, *const TWrite)>,
}

impl<R, W> ProcessFrom<R, W> {
	pub fn new() -> Self {
		Self {
			_marker: PhantomData,
		}
	}
}

impl<R: ReadableStorage, W: WritableStorage> Process for ProcessFrom<R, W> where
	<<<R as ReadableStorage>::Read as RefLike>::Borrowed as BorrowedStorage>::Batch : f64
{ // &'static [R]

}
*/