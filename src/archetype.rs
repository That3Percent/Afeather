use crate::*;
use std::rc::Rc;

pub struct Archetype {
    num_entities: usize,
    // TODO: Not pub
    pub components: Components,
}

impl Archetype {
    pub fn new() -> Self {
        Self {
            num_entities: 0,
            components: Components::new(),
        }
    }

    pub fn num_entities(&self) -> usize {
        self.num_entities
    }

    pub fn entity_write_slot(&mut self) -> usize {
        let result = self.num_entities;
        self.num_entities += 1;
        result
    }

    pub fn get_storage<T: AnyStorage>(&self) -> Option<Rc<T>> {
        self.components.get_storage::<T>()
    }

    pub fn get_storage_mut<T: AnyStorage>(&self) -> Option<Rc<T>> {
        self.components.get_storage_mut::<T>()
    }

    pub fn add_component<T: Component>(&mut self, storage: T::Storage) {
        self.components.add(storage);
    }

    // TODO: Take ReadableStorage
    /*
    pub fn read_component<T: Component>(&self, index: usize) -> Option<&<<T as Component>::Storage as ReadableStorage>::Read> where T::Storage : ReadableStorage<Read=T> {
        T::Storage::read(self, index)
    }

    pub fn read_component_mut<T: Component>(&self, index: usize) -> Option<&mut <<T as Component>::Storage as ReadableStorage>::Read> where T::Storage : WritableStorage<Read=T> {
        T::Storage::read_mut(self, index)
    }

    pub fn read_component_batch<T: Component>(&self) -> Option<&<<T as Component>::Storage as ReadableStorage>::BatchRead> where T::Storage : ReadableStorage<Read=T> {
        T::Storage::read_batch(self)
    }
    */
}

pub trait ArchetypeInitializer {
    fn initialize(self, archetype: &mut Archetype);
}

pub trait ArchetypeFilter {
    fn includes(&self, archetype: &Archetype) -> bool;
}

pub trait EntityWriter {
    fn write(self, archetype: &mut Archetype, index: usize);
}

// TODO: The design here uses a static method, because we want to be able to simply specify the read/write
// types of a query as an associated type. But, that offers a little less flexibility then just having
// execute take the world and use some kind of filter builder. That would allow, eg: specifying per-archetype
// components having a particular value or the like. The slight distinction that's interesting to performance
// for that case is that the archetype and components would not need to be borrowed just to be filtered out,
// allowing a higher degree of parallelism. This concern does not outweigh simplicity for now.

impl ReadableStorage for () {
    type Read = ();
    #[inline(always)]
    fn get(_world_storage: &Components, _archetype_storage: &Components) -> Option<Self::Read> {
        Some(())
    }
}

impl RefLike for () {
    type Borrowed = ();
    #[inline(always)]
    fn borrow(&self) -> Self::Borrowed {}
}

impl<'a> BorrowedStorage for () {
    type Item = ();
    type Batch = ();
    fn version(&self) -> Version {
        Version(0)
    }
    fn read(&self, _index: usize) -> Option<Self::Item> {
        Some(())
    }
    fn read_batch(&self) -> Self::Batch {}
}

impl<T: ReadableStorage> ReadableStorage for Option<T> {
    type Read = Option<T::Read>;
    #[inline(always)]
    fn get(world_storage: &Components, archetype_storage: &Components) -> Option<Self::Read> {
        Some(T::get(world_storage, archetype_storage))
    }
}

impl<T: RefLike> RefLike for Option<T> {
    type Borrowed = Option<T::Borrowed>;
    fn borrow(&self) -> Self::Borrowed {
        self.as_ref().map(|v| v.borrow())
    }
}

impl<T: BorrowedStorage> BorrowedStorage for Option<T> {
    type Item = Option<T::Item>;
    type Batch = Option<T::Batch>;
    #[inline(always)]
    fn version(&self) -> Version {
        match self {
            Some(storage) => storage.version(),
            None => Version(0),
        }
    }
    fn read(&self, index: usize) -> Option<Self::Item> {
        self.as_ref().map(|s| s.read(index))
    }
    fn read_batch(&self) -> Self::Batch {
        self.as_ref().map(|s| s.read_batch())
    }
}

impl<T: ComponentWrite> ComponentWrite for Option<T> {
    type BatchWrite = Option<T::BatchWrite>;
    fn read_batch_mut(archetype: &Archetype) -> Option<Self::BatchWrite> {
        Some(T::read_batch_mut(archetype))
    }
}

pub trait ComponentWrite {
    type BatchWrite: Sized;

    fn read_batch_mut(archetype: &Archetype) -> Option<Self::BatchWrite>;
}
