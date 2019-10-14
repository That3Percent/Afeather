use crate::*;
use extend_lifetime::extend_lifetime;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

pub struct Global<T> {
    cell: RefCell<BorrowedGlobal<T>>,
}

pub struct BorrowedGlobal<T> {
    version: Version,
    value: T,
}

impl<T> BorrowedGlobal<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            version: Version(0),
        }
    }
}

impl<T: 'static> BorrowedStorage for BorrowedGlobal<T> {
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

impl<T> Global<T> {
    pub fn new(value: T) -> Self {
        let cell = RefCell::new(BorrowedGlobal::new(value));
        Self { cell }
    }
}

impl<T: 'static> AnyStorage for Global<T> {}

impl<T: 'static> ReadableStorage for Global<T> {
    type Read = Rc<Self>;
    #[inline(always)]
    fn get(world_storage: &Components, _archetype_storage: &Components) -> Option<Self::Read> {
        world_storage.get_storage::<Self>()
    }
}

impl<T: 'static> RefLike for Global<T> {
    type Borrowed = Ref<'static, BorrowedGlobal<T>>;
    fn borrow(&self) -> Self::Borrowed {
        // See also 0a427633-4da0-4729-bae6-45d77542261c
        unsafe { extend_lifetime(self.cell.borrow()) }
    }
}
