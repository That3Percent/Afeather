use crate::*;
use extend_lifetime::extend_lifetime;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

// This is quite a bit smaller than one might expect from another ECS,
// but is smaller to balance the cost that there is a larger cardinality
// of archetypes due to PerArchetype storage
const CAPACITY: usize = 4096;

pub struct BorrowedPerEntity<T> {
    values: Vec<T>,
    version: Version,
}

impl<T> BorrowedPerEntity<T> {
    pub fn new() -> Self {
        Self {
            values: Vec::with_capacity(CAPACITY),
            version: Version(0),
        }
    }
}

impl<T: 'static> BorrowedStorage for BorrowedPerEntity<T> {
    type Item = &'static T;
    type Batch = &'static [T];
    #[inline(always)]
    fn version(&self) -> Version {
        self.version
    }
    #[inline(always)]
    fn read(&self, index: usize) -> Option<&'static T> {
        let borrow = Some(if cfg!(debug_assertions) {
            &self.values[index]
        } else {
            unsafe { self.values.get_unchecked(index) }
        });
        // See also 0a427633-4da0-4729-bae6-45d77542261c
        unsafe { extend_lifetime(borrow) }
    }
    #[inline(always)]
    fn read_batch(&self) -> Self::Batch {
        // See also 0a427633-4da0-4729-bae6-45d77542261c
        unsafe { extend_lifetime(&self.values[..]) }
    }
}

impl<T: 'static> BorrowedStorageMut for BorrowedPerEntity<T> {
    type ItemMut = T;
    type BatchMut = &'static mut [T];
    #[inline(always)]
    fn write(&mut self, index: usize, item: Self::ItemMut) {
        // TODO: Unchecked in release
        self.values[index] = item;
    }
    #[inline(always)]
    fn write_batch(&mut self) -> Self::BatchMut {
        let borrow = &mut self.values[..];
        // See also 0a427633-4da0-4729-bae6-45d77542261c
        unsafe { extend_lifetime(borrow) }
    }
}

pub struct PerEntity<T> {
    cell: RefCell<BorrowedPerEntity<T>>,
}

impl<T> PerEntity<T> {
    pub fn new() -> Self {
        let cell = RefCell::new(BorrowedPerEntity::new());
        Self { cell }
    }
}

impl<T: 'static> AnyStorage for PerEntity<T> {
	fn remove_entity(&self, index: usize, top: usize) {
		let mut borrow = self.borrow_mut();
		borrow.values.swap_remove(index);
		debug_assert!(top == borrow.values.len())
	}
}

impl<T: 'static> ReadableStorage for PerEntity<T> {
    type Read = Rc<Self>;
    fn get(_world_storage: &Components, archetype_storage: &Components) -> Option<Self::Read> {
        archetype_storage.get_storage::<Self>()
    }
}

impl<T: 'static> RefLike for PerEntity<T> {
    type Borrowed = Ref<'static, BorrowedPerEntity<T>>;
    fn borrow(&self) -> Self::Borrowed {
        // See also 0a427633-4da0-4729-bae6-45d77542261c
        unsafe { extend_lifetime(self.cell.borrow()) }
    }
}

impl<T: 'static> WritableStorage for PerEntity<T> {
    type ReadMut = Rc<Self>;
    fn get_mut(archetype: &Archetype) -> Option<Self::ReadMut> {
        archetype.get_storage_mut()
    }
}

impl<T: 'static> RefLikeMut for PerEntity<T> {
    type BorrowedMut = RefMut<'static, BorrowedPerEntity<T>>;
    fn borrow_mut(&self) -> Self::BorrowedMut {
        // See also 0a427633-4da0-4729-bae6-45d77542261c
        unsafe { extend_lifetime(self.cell.borrow_mut()) }
    }
}

impl<T: EntityWriter + Component<Storage = PerEntity<T>>> ArchetypeInitializerFromComponentStorage
    for PerEntity<T>
{
    type Component = T;
    fn initialize(component: T, archetype: &mut Archetype) {
        let storage = Self::new();
        archetype.add_component::<T>(storage);
        component.write(archetype, 0);
    }
}

impl<T: Component<Storage = PerEntity<T>>> ArchetypeFilterFromComponentStorage for PerEntity<T> {
    type Component = T;
    fn includes(_component: &T, archetype: &Archetype) -> bool {
        archetype.get_storage::<Self>().is_some()
        /*
        // TODO: We want to not write when at capacity, but
        // if implemented in this way it would lead to a bug if
        // this method is used to re-write an existing entity.
        // Starting conservative to avoid bugs for the moment.
        if let Some(storage) = archetype.get_storage::<T>() {
            storage.values.borrow().len() < CAPACITY
        } else {
            false
        }
        */
    }
}

impl<T: Component<Storage = PerEntity<T>>> EntityWriterFromComponentStorage for PerEntity<T> {
    type Component = T;
    #[inline(always)]
    fn write(component: T, archetype: &mut Archetype, index: usize) {
        let storage = archetype.get_storage_mut::<Self>().unwrap();
        let values = &mut storage.cell.borrow_mut().values;
        if index == values.len() {
            values.push(component)
        } else {
            values[index] = component;
        }
    }
}
