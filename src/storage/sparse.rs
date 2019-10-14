use crate::*;
use extend_lifetime::extend_lifetime;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

pub struct Sparse<T> {
    cell: RefCell<BorrowedSparse<T>>,
}

pub struct BorrowedSparse<T> {
    version: Version,
    values: HashMap<usize, T>,
}

impl<T> BorrowedSparse<T> {
    pub fn new() -> Self {
        Self {
            version: Version(0),
            values: HashMap::new(),
        }
    }
}

impl<T: 'static> BorrowedStorage for BorrowedSparse<T> {
    type Item = &'static T;
    type Batch = &'static HashMap<usize, T>;
    fn version(&self) -> Version {
        self.version
    }
    fn read(&self, index: usize) -> Option<&'static T> {
        // See also 0a427633-4da0-4729-bae6-45d77542261c
        unsafe { extend_lifetime(self.values.get(&index)) }
    }
    fn read_batch(&self) -> Self::Batch {
        // See also 0a427633-4da0-4729-bae6-45d77542261c
        unsafe { extend_lifetime(&self.values) }
    }
}

impl<T> Sparse<T> {
    pub fn new() -> Self {
        let cell = RefCell::new(BorrowedSparse::new());
        Self { cell }
    }
}

impl<T: 'static> AnyStorage for Sparse<T> {}

impl<T: Component> ReadableStorage for Sparse<T> {
    type Read = Rc<Self>;
    fn get(_world_storage: &Components, archetype_storage: &Components) -> Option<Rc<Self>> {
        archetype_storage.get_storage::<Self>()
    }
}

impl<T: 'static> RefLike for Sparse<T> {
    type Borrowed = Ref<'static, BorrowedSparse<T>>;
    fn borrow(&self) -> Self::Borrowed {
        // See also 0a427633-4da0-4729-bae6-45d77542261c
        unsafe { extend_lifetime(self.cell.borrow()) }
    }
}
/*
impl<T: Component> WritableStorage for Sparse<T> {
    /*
    fn read_mut(archetype: &Archetype, index: usize) -> Option<&mut Self::Read> {
        archetype.get_storage::<Self>().and_then(|s| unsafe { (&mut *s.values.get()) }.get_mut(&index))
    }
    fn read_batch_mut(archetype: &Archetype) -> Option<&mut Self::BatchRead> {
        archetype.get_storage::<Self>().map(|s| unsafe { (&mut *s.values.get()) } )
    }
    */
}
*/

impl<T: Component<Storage = Sparse<T>>> ArchetypeFilterFromComponentStorage for Sparse<T> {
    type Component = T;
    #[inline]
    fn includes(_component: &T, _archetype: &Archetype) -> bool {
        true
    }
}

impl<T: Component<Storage = Sparse<T>>> EntityWriterFromComponentStorage for Sparse<T> {
    type Component = T;
    #[inline]
    fn write(component: T, archetype: &mut Archetype, index: usize) {
        let s = archetype.get_storage_mut::<Self>().unwrap();
        s.cell.borrow_mut().values.insert(index, component);
    }
}

impl<T: EntityWriter + Component<Storage = Sparse<T>>> ArchetypeInitializerFromComponentStorage
    for Sparse<T>
{
    type Component = T;
    fn initialize(component: T, archetype: &mut Archetype) {
        let storage = Self::new();
        archetype.add_component::<T>(storage);
        component.write(archetype, 0);
    }
}
