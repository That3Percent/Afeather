use crate::*;
use extend_lifetime::extend_lifetime;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

pub struct PerArchetype<T> {
    cell: RefCell<BorrowedPerArchetype<T>>,
}

pub struct BorrowedPerArchetype<T> {
    version: Version,
    value: T,
}

impl<T> BorrowedPerArchetype<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            version: Version(0),
        }
    }
}

impl<T: 'static> BorrowedStorage for BorrowedPerArchetype<T> {
    type Item = &'static T;
    type Batch = &'static T;
    #[inline(always)]
    fn read(&self, _index: usize) -> Option<Self::Item> {
        // See also 0a427633-4da0-4729-bae6-45d77542261c
        unsafe { extend_lifetime(Some(&self.value)) }
    }
    fn read_batch(&self) -> Self::Batch {
        // See also 0a427633-4da0-4729-bae6-45d77542261c
        unsafe { extend_lifetime(&self.value) }
    }
    #[inline(always)]
    fn version(&self) -> Version {
        self.version
    }
}

impl<T> PerArchetype<T> {
    pub fn new(value: T) -> Self {
        let cell = RefCell::new(BorrowedPerArchetype::new(value));
        Self { cell }
    }
}

impl<T: 'static> AnyStorage for PerArchetype<T> {}

impl<T: 'static> ReadableStorage for PerArchetype<T> {
    type Read = Rc<Self>;
    fn get(_world_storage: &Components, archetype_storage: &Components) -> Option<Self::Read> {
        archetype_storage.get_storage::<Self>()
    }
}

impl<T: 'static> RefLike for PerArchetype<T> {
    type Borrowed = Ref<'static, BorrowedPerArchetype<T>>;
    fn borrow(&self) -> Self::Borrowed {
        // See also 0a427633-4da0-4729-bae6-45d77542261c
        unsafe { extend_lifetime(self.cell.borrow()) }
    }
}

impl<T: Component<Storage = PerArchetype<T>>> ArchetypeInitializerFromComponentStorage
    for PerArchetype<T>
{
    type Component = T;
    fn initialize(component: T, archetype: &mut Archetype) {
        let storage = Self::new(component); // FIXME Version
        archetype.add_component::<T>(storage);
    }
}

impl<T: Component<Storage = PerArchetype<T>> + Eq> ArchetypeFilterFromComponentStorage
    for PerArchetype<T>
{
    type Component = T;
    fn includes(component: &T, archetype: &Archetype) -> bool {
        let storage = archetype.get_storage::<Self>();
        if let Some(storage) = storage {
            &storage.borrow().value == component
        } else {
            false
        }
    }
}

impl<T: Component<Storage = PerArchetype<T>>> EntityWriterFromComponentStorage for PerArchetype<T> {
    type Component = T;
    #[inline]
    fn write(_component: T, _archetype: &mut Archetype, _index: usize) {}
}
