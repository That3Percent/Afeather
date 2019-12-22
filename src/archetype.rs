use crate::*;
use std::rc::Rc;
use unordered_hash::UnorderedHasher;

pub struct Archetype {
    num_entities: usize,
    components: Components,
	requirements: u64,
}

impl Archetype {
    pub fn new(requirements: u64) -> Self {
        Self {
            num_entities: 0,
            components: Components::new(),
			requirements
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

	pub fn remove_entity(&mut self, index: usize) {
		self.num_entities -= 1;
		let top = self.num_entities;
		self.components.remove_entity(index, top);
	}

    pub fn get_storage<T: AnyStorage>(&self) -> Option<Rc<T>> {
        self.components.get_storage::<T>()
    }

    pub fn get_storage_mut<T: AnyStorage>(&self) -> Option<Rc<T>> {
        self.components.get_storage_mut::<T>()
    }

    pub fn add_storage<T: Component>(&mut self, storage: T::Storage) {
        self.components.add(storage);
    }

	pub fn get_requirements(&self) -> u64 {
		self.requirements
	}

	pub fn components(&self) -> &Components {
		&self.components
	}
}

pub trait ArchetypeInitializer {
    fn initialize(self, archetype: &mut Archetype);
}

pub trait ArchetypeFilter {
    fn includes(&self, archetype: &Archetype) -> bool;
}

pub trait EntityWriter {
    fn write(self, archetype: &mut Archetype, index: usize);
	fn add_archetype_requirements(&self, hasher: &mut UnorderedHasher);
}

// TODO: The design here uses a static method, because we want to be able to simply specify the read/write
// types of a query as an associated type. But, that offers a little less flexibility then just having
// execute take the world and use some kind of filter builder. That would allow, eg: specifying per-archetype
// components having a particular value or the like. The slight distinction that's interesting to performance
// for that case is that the archetype and components would not need to be borrowed just to be filtered out,
// allowing a higher degree of parallelism. This concern does not outweigh simplicity for now.


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
