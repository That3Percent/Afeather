pub use crate::*;

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
