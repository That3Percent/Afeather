use crate::*;
use std::cmp::max;

// TODO: Macros

/// REF_LIKE ///
impl<T0: RefLike, T1: RefLike> RefLike for (T0, T1) {
    type Borrowed = (T0::Borrowed, T1::Borrowed);
    fn borrow(&self) -> Self::Borrowed {
        (self.0.borrow(), self.1.borrow())
    }
}

impl<T0: RefLike, T1: RefLike, T2: RefLike> RefLike for (T0, T1, T2) {
    type Borrowed = (T0::Borrowed, T1::Borrowed, T2::Borrowed);
    fn borrow(&self) -> Self::Borrowed {
        (self.0.borrow(), self.1.borrow(), self.2.borrow())
    }
}

impl<T0: RefLike, T1: RefLike, T2: RefLike, T3: RefLike> RefLike for (T0, T1, T2, T3) {
    type Borrowed = (T0::Borrowed, T1::Borrowed, T2::Borrowed, T3::Borrowed);
    fn borrow(&self) -> Self::Borrowed {
        (
            self.0.borrow(),
            self.1.borrow(),
            self.2.borrow(),
            self.3.borrow(),
        )
    }
}

/// REF_LIKE_MUT ///

impl<T0: RefLikeMut, T1: RefLikeMut> RefLikeMut for (T0, T1) {
    type BorrowedMut = (T0::BorrowedMut, T1::BorrowedMut);
    fn borrow_mut(&self) -> Self::BorrowedMut {
        (self.0.borrow_mut(), self.1.borrow_mut())
    }
}

impl<T0: RefLikeMut, T1: RefLikeMut, T2: RefLikeMut> RefLikeMut for (T0, T1, T2) {
    type BorrowedMut = (T0::BorrowedMut, T1::BorrowedMut, T2::BorrowedMut);
    fn borrow_mut(&self) -> Self::BorrowedMut {
        (
            self.0.borrow_mut(),
            self.1.borrow_mut(),
            self.2.borrow_mut(),
        )
    }
}

impl<T0: RefLikeMut, T1: RefLikeMut, T2: RefLikeMut, T3: RefLikeMut> RefLikeMut
    for (T0, T1, T2, T3)
{
    type BorrowedMut = (
        T0::BorrowedMut,
        T1::BorrowedMut,
        T2::BorrowedMut,
        T3::BorrowedMut,
    );
    fn borrow_mut(&self) -> Self::BorrowedMut {
        (
            self.0.borrow_mut(),
            self.1.borrow_mut(),
            self.2.borrow_mut(),
            self.3.borrow_mut(),
        )
    }
}

/// BorrowedStorage ///

impl<T0: BorrowedStorage, T1: BorrowedStorage> BorrowedStorage for (T0, T1) {
    type Item = (T0::Item, T1::Item);
    type Batch = (T0::Batch, T1::Batch);
    fn version(&self) -> Version {
        max(self.0.version(), self.1.version())
    }
    fn read(&self, index: usize) -> Option<Self::Item> {
        self.0
            .read(index)
            .and_then(|t0| self.1.read(index).map(|t1| (t0, t1)))
    }
    fn read_batch(&self) -> Self::Batch {
        (self.0.read_batch(), self.1.read_batch())
    }
}

impl<T0: BorrowedStorage, T1: BorrowedStorage, T2: BorrowedStorage> BorrowedStorage
    for (T0, T1, T2)
{
    type Item = (T0::Item, T1::Item, T2::Item);
    type Batch = (T0::Batch, T1::Batch, T2::Batch);
    fn version(&self) -> Version {
        max(max(self.0.version(), self.1.version()), self.2.version())
    }
    fn read(&self, index: usize) -> Option<Self::Item> {
        self.0.read(index).and_then(|t0| {
            self.1
                .read(index)
                .and_then(|t1| self.2.read(index).map(|t2| (t0, t1, t2)))
        })
    }
    fn read_batch(&self) -> Self::Batch {
        (
            self.0.read_batch(),
            self.1.read_batch(),
            self.2.read_batch(),
        )
    }
}

impl<T0: BorrowedStorage, T1: BorrowedStorage, T2: BorrowedStorage, T3: BorrowedStorage>
    BorrowedStorage for (T0, T1, T2, T3)
{
    type Item = (T0::Item, T1::Item, T2::Item, T3::Item);
    type Batch = (T0::Batch, T1::Batch, T2::Batch, T3::Batch);
    fn version(&self) -> Version {
        max(
            max(max(self.0.version(), self.1.version()), self.2.version()),
            self.3.version(),
        )
    }
    fn read(&self, index: usize) -> Option<Self::Item> {
        self.0.read(index).and_then(|t0| {
            self.1.read(index).and_then(|t1| {
                self.2
                    .read(index)
                    .and_then(|t2| self.3.read(index).map(|t3| (t0, t1, t2, t3)))
            })
        })
    }
    fn read_batch(&self) -> Self::Batch {
        (
            self.0.read_batch(),
            self.1.read_batch(),
            self.2.read_batch(),
            self.3.read_batch(),
        )
    }
}

/// BorrowedStorageMut ///

impl<T0: BorrowedStorageMut, T1: BorrowedStorageMut> BorrowedStorageMut for (T0, T1) {
    type ItemMut = (T0::ItemMut, T1::ItemMut);
    type BatchMut = (T0::BatchMut, T1::BatchMut);
    fn write(&mut self, index: usize, item: Self::ItemMut) {
        self.0.write(index, item.0);
        self.1.write(index, item.1);
    }
    fn write_batch(&mut self) -> Self::BatchMut {
        (self.0.write_batch(), self.1.write_batch())
    }
}

impl<T0: BorrowedStorageMut, T1: BorrowedStorageMut, T2: BorrowedStorageMut> BorrowedStorageMut
    for (T0, T1, T2)
{
    type ItemMut = (T0::ItemMut, T1::ItemMut, T2::ItemMut);
    type BatchMut = (T0::BatchMut, T1::BatchMut, T2::BatchMut);
    fn write(&mut self, index: usize, item: Self::ItemMut) {
        self.0.write(index, item.0);
        self.1.write(index, item.1);
        self.2.write(index, item.2);
    }
    fn write_batch(&mut self) -> Self::BatchMut {
        (
            self.0.write_batch(),
            self.1.write_batch(),
            self.2.write_batch(),
        )
    }
}

impl<
        T0: BorrowedStorageMut,
        T1: BorrowedStorageMut,
        T2: BorrowedStorageMut,
        T3: BorrowedStorageMut,
    > BorrowedStorageMut for (T0, T1, T2, T3)
{
    type ItemMut = (T0::ItemMut, T1::ItemMut, T2::ItemMut, T3::ItemMut);
    type BatchMut = (T0::BatchMut, T1::BatchMut, T2::BatchMut, T3::BatchMut);
    fn write(&mut self, index: usize, item: Self::ItemMut) {
        self.0.write(index, item.0);
        self.1.write(index, item.1);
        self.2.write(index, item.2);
        self.3.write(index, item.3);
    }
    fn write_batch(&mut self) -> Self::BatchMut {
        (
            self.0.write_batch(),
            self.1.write_batch(),
            self.2.write_batch(),
            self.3.write_batch(),
        )
    }
}

//// ARCHETYPE_FILTER ////

impl<T0: ArchetypeFilter, T1: ArchetypeFilter> ArchetypeFilter for (T0, T1) {
    fn includes(&self, archetype: &Archetype) -> bool {
        self.0.includes(archetype) && self.1.includes(archetype)
    }
}

impl<T0: ArchetypeFilter, T1: ArchetypeFilter, T2: ArchetypeFilter> ArchetypeFilter
    for (T0, T1, T2)
{
    fn includes(&self, archetype: &Archetype) -> bool {
        self.0.includes(archetype) && self.1.includes(archetype) && self.2.includes(archetype)
    }
}

impl<T0: ArchetypeFilter, T1: ArchetypeFilter, T2: ArchetypeFilter, T3: ArchetypeFilter>
    ArchetypeFilter for (T0, T1, T2, T3)
{
    fn includes(&self, archetype: &Archetype) -> bool {
        self.0.includes(archetype)
            && self.1.includes(archetype)
            && self.2.includes(archetype)
            && self.3.includes(archetype)
    }
}

//// ARCHETYPE_INITIALIZER ////

impl<T0: ArchetypeInitializer, T1: ArchetypeInitializer> ArchetypeInitializer for (T0, T1) {
    fn initialize(self, archetype: &mut Archetype) {
        let (t0, t1) = self;
        t0.initialize(archetype);
        t1.initialize(archetype);
    }
}

impl<T0: ArchetypeInitializer, T1: ArchetypeInitializer, T2: ArchetypeInitializer>
    ArchetypeInitializer for (T0, T1, T2)
{
    fn initialize(self, archetype: &mut Archetype) {
        let (t0, t1, t2) = self;
        t1.initialize(archetype);
        t2.initialize(archetype);
        t0.initialize(archetype);
    }
}

impl<
        T0: ArchetypeInitializer,
        T1: ArchetypeInitializer,
        T2: ArchetypeInitializer,
        T3: ArchetypeInitializer,
    > ArchetypeInitializer for (T0, T1, T2, T3)
{
    fn initialize(self, archetype: &mut Archetype) {
        let (t0, t1, t2, t3) = self;
        t0.initialize(archetype);
        t1.initialize(archetype);
        t2.initialize(archetype);
        t3.initialize(archetype);
    }
}

//// ENTITY_WRITER ////

impl<T0: EntityWriter, T1: EntityWriter> EntityWriter for (T0, T1) {
    fn write(self, archetype: &mut Archetype, index: usize) {
        let (t0, t1) = self;
        t0.write(archetype, index);
        t1.write(archetype, index);
    }
}

impl<T0: EntityWriter, T1: EntityWriter, T2: EntityWriter> EntityWriter for (T0, T1, T2) {
    fn write(self, archetype: &mut Archetype, index: usize) {
        let (t0, t1, t2) = self;
        t0.write(archetype, index);
        t1.write(archetype, index);
        t2.write(archetype, index);
    }
}

impl<T0: EntityWriter, T1: EntityWriter, T2: EntityWriter, T3: EntityWriter> EntityWriter
    for (T0, T1, T2, T3)
{
    fn write(self, archetype: &mut Archetype, index: usize) {
        let (t0, t1, t2, t3) = self;
        t1.write(archetype, index);
        t2.write(archetype, index);
        t0.write(archetype, index);
        t3.write(archetype, index);
    }
}

//// READABLE_STORAGE ////
impl<T0: ReadableStorage, T1: ReadableStorage> ReadableStorage for (T0, T1) {
    type Read = (T0::Read, T1::Read);
    #[inline(always)]
    fn get(world_storage: &Components, archetype_storage: &Components) -> Option<Self::Read> {
        T0::get(world_storage, archetype_storage)
            .and_then(|t0| T1::get(world_storage, archetype_storage).map(|t1| (t0, t1)))
    }
}

impl<T0: ReadableStorage, T1: ReadableStorage, T2: ReadableStorage> ReadableStorage
    for (T0, T1, T2)
{
    type Read = (T0::Read, T1::Read, T2::Read);
    fn get(world_storage: &Components, archetype_storage: &Components) -> Option<Self::Read> {
        T0::get(world_storage, archetype_storage).and_then(|t0| {
            T1::get(world_storage, archetype_storage)
                .and_then(|t1| T2::get(world_storage, archetype_storage).map(|t2| (t0, t1, t2)))
        })
    }
}

impl<T0: ReadableStorage, T1: ReadableStorage, T2: ReadableStorage, T3: ReadableStorage>
    ReadableStorage for (T0, T1, T2, T3)
{
    type Read = (T0::Read, T1::Read, T2::Read, T3::Read);
    fn get(world_storage: &Components, archetype_storage: &Components) -> Option<Self::Read> {
        T0::get(world_storage, archetype_storage).and_then(|t0| {
            T1::get(world_storage, archetype_storage).and_then(|t1| {
                T2::get(world_storage, archetype_storage).and_then(|t2| {
                    T3::get(world_storage, archetype_storage).map(|t3| (t0, t1, t2, t3))
                })
            })
        })
    }
}
