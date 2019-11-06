use crate::*;
use downcast_rs::Downcast;
use extend_lifetime::extend_lifetime;
use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub trait AnyStorage: Downcast {
	fn remove_entity(&self, index: usize, top: usize);
}

impl_downcast!(AnyStorage);

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Version(pub u64);

pub trait RefLike {
    // TODO: There's no reason for this trait's associated type to require BorrowedStorage,
    // but in ReadableStorage we need to specify this (and that's the only place this is used now)
    // See also c1d1ffbe-1226-41ed-9190-6e8c32ccdced
    type Borrowed: BorrowedStorage;
    fn borrow(&self) -> Self::Borrowed;
}

impl<T: ReadableStorage> ReadableStorage for Rc<T> {
    type Read = T::Read;
    #[inline(always)]
    fn get(world_storage: &Components, archetype_storage: &Components) -> Option<Self::Read> {
        T::get(world_storage, archetype_storage)
    }
}

impl<T: RefLike> RefLike for Rc<T> {
    type Borrowed = T::Borrowed;
    fn borrow(&self) -> Self::Borrowed {
        self.deref().borrow()
    }
}

impl<T: RefLikeMut> RefLikeMut for Rc<T> {
    type BorrowedMut = T::BorrowedMut;
    fn borrow_mut(&self) -> Self::BorrowedMut {
        Deref::deref(self).borrow_mut()
    }
}

pub trait RefLikeMut {
    // See also c1d1ffbe-1226-41ed-9190-6e8c32ccdced
    type BorrowedMut: BorrowedStorageMut;
    fn borrow_mut(&self) -> Self::BorrowedMut;
}

impl<T: BorrowedStorage + 'static> RefLike for RefCell<T> {
    type Borrowed = Ref<'static, T>;
    #[inline(always)]
    fn borrow(&self) -> Self::Borrowed {
        // We can't get a borrow with references from self without GAT, so we
        // very unsafely lie to the compiler about the nature of our references. :/
        // See http://lukaskalbertodt.github.io/2018/08/03/solving-the-generalized-streaming-iterator-problem-without-gats.html#summary
        // See http://smallcultfollowing.com/babysteps/blog/2016/11/02/associated-type-constructors-part-1-basic-concepts-and-introduction/
        // See the other parts of the previous blog post
        // See also 0a427633-4da0-4729-bae6-45d77542261c
        unsafe { extend_lifetime(self.borrow()) }
    }
}

impl<B: BorrowedStorage> BorrowedStorage for Ref<'static, B> {
    type Item = B::Item;
    type Batch = B::Batch;
    #[inline(always)]
    fn version(&self) -> Version {
        self.deref().version()
    }
    #[inline(always)]
    fn read(&self, index: usize) -> Option<Self::Item> {
        self.deref().read(index)
    }
    #[inline(always)]
    fn read_batch(&self) -> Self::Batch {
        self.deref().read_batch()
    }
}

impl<T: BorrowedStorageMut + 'static> RefLikeMut for RefCell<T> {
    type BorrowedMut = RefMut<'static, T>;
    fn borrow_mut(&self) -> Self::BorrowedMut {
        // See also 0a427633-4da0-4729-bae6-45d77542261c
        unsafe { extend_lifetime(self.borrow_mut()) }
    }
}

impl<T: BorrowedStorageMut + 'static> BorrowedStorageMut for RefMut<'static, T> {
    type ItemMut = T::ItemMut;
    type BatchMut = T::BatchMut;
    #[inline(always)]
    fn write(&mut self, index: usize, item: Self::ItemMut) {
        self.deref_mut().write(index, item)
    }
    #[inline(always)]
    fn write_batch(&mut self) -> Self::BatchMut {
        self.deref_mut().write_batch()
    }
}

pub trait BorrowedStorage {
    type Item;
    type Batch;
    fn version(&self) -> Version;
    fn read(&self, index: usize) -> Option<Self::Item>;
    fn read_batch(&self) -> Self::Batch;
}

pub trait BorrowedStorageMut {
    type ItemMut;
    type BatchMut;
    fn write(&mut self, index: usize, item: Self::ItemMut);
    fn write_batch(&mut self) -> Self::BatchMut;
}

pub trait ReadableStorage {
    // TODO: Associated type bound, Borrow=BorrowedStorage. See also c1d1ffbe-1226-41ed-9190-6e8c32ccdced
    type Read: RefLike;
    fn get(world_storage: &Components, archetype_storage: &Components) -> Option<Self::Read>;
}

pub trait WritableStorage: ReadableStorage {
    type ReadMut: RefLikeMut;
    fn get_mut(archetype: &Archetype) -> Option<Self::ReadMut>;
}

pub trait EntityStorage {

}

// This and the other similar redirection traits are because the compiler is complaining about duplicate implementations of the trait
// if we write ArchetypeInitializer for the components directly. This doesn't seem right, since the trait bounds exclude to a single
// concrete type.
pub trait ArchetypeInitializerFromComponentStorage {
    type Component;
    fn initialize(component: Self::Component, archetype: &mut Archetype);
}

impl<
        S: ArchetypeInitializerFromComponentStorage<Component = T> + ReadableStorage + AnyStorage,
        T: Component<Storage = S>,
    > ArchetypeInitializer for T
{
    #[inline]
    fn initialize(self, archetype: &mut Archetype) {
        S::initialize(self, archetype)
    }
}

pub trait ArchetypeFilterFromComponentStorage {
    type Component;
    fn includes(component: &Self::Component, archetype: &Archetype) -> bool;
}

impl<
        S: ArchetypeFilterFromComponentStorage<Component = T> + ReadableStorage + AnyStorage,
        T: Component<Storage = S>,
    > ArchetypeFilter for T
{
    #[inline]
    fn includes(&self, archetype: &Archetype) -> bool {
        S::includes(self, archetype)
    }
}

pub trait EntityWriterFromComponentStorage {
    type Component;
    fn write(component: Self::Component, archetype: &mut Archetype, index: usize);
}

impl<
        S: EntityWriterFromComponentStorage<Component = T> + ReadableStorage + AnyStorage,
        T: Component<Storage = S>,
    > EntityWriter for T
{
    #[inline]
    fn write(self, archetype: &mut Archetype, index: usize) {
        S::write(self, archetype, index)
    }
}

mod per_entity;
pub use per_entity::*;
mod per_archetype;
pub use per_archetype::*;
mod sparse;
pub use sparse::*;
mod global;
pub use global::*;
mod components;
pub use components::*;
