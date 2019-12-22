#![allow(non_snake_case, unused_variables, unused_mut)]

use crate::*;
use std::cmp::max;
use unordered_hash::UnorderedHasher;

macro_rules! tuple {
	($($T:ident),*) => {
		impl<$($T: RefLike,)*> RefLike for ($($T,)*) {
			type Borrowed = ($($T::Borrowed,)*);
			fn borrow(&self) -> Self::Borrowed {
				let ($($T,)*) = self;
				($($T.borrow(),)*)
			}
		}

		impl<$($T: RefLikeMut,)*> RefLikeMut for ($($T,)*) {
			type BorrowedMut = ($($T::BorrowedMut,)*);
			fn borrow_mut(&self) -> Self::BorrowedMut {
				let ($($T,)*) = self;
				($($T.borrow_mut(),)*)
			}
		}

		impl<$($T: BorrowedStorage,)*> BorrowedStorage for ($($T,)*) {
			type Item = ($($T::Item,)*);
			type Batch = ($($T::Batch,)*);
			fn version(&self) -> Version {
				let mut v = Version(0);
				let ($($T,)*) = self;
				$(v = max(v, $T.version());)*
				v
			}

			fn read(&self, index: usize) -> Option<Self::Item> {
				let ($($T,)*) = self;
				$(let $T = $T.read(index)?;)*
				Some(($($T,)*))
			}

			fn read_batch(&self) -> Self::Batch {
				let ($($T,)*) = self;
				($($T.read_batch(),)*)
			}
		}

		impl<$($T: ArchetypeFilter,)*> ArchetypeFilter for ($($T,)*) {
			fn includes(&self, archetype: &Archetype) -> bool {
				let ($($T,)*) = self;
				true $(&& $T.includes(archetype))*
			}
		}

		impl<$($T: ArchetypeInitializer,)*> ArchetypeInitializer for ($($T,)*) {
			fn initialize(self, archetype: &mut Archetype) {
				let ($($T,)*) = self;
				$($T.initialize(archetype);)*
			}
		}

		impl<$($T: EntityWriter,)*> EntityWriter for ($($T,)*) {
			fn write(self, archetype: &mut Archetype, index: usize) {
				let ($($T,)*) = self;
				$($T.write(archetype, index);)*
			}

			fn add_archetype_requirements(&self, hasher: &mut UnorderedHasher) {
				let ($($T,)*) = self;
				$($T.add_archetype_requirements(hasher);)*
			}
		}

		impl<$($T: ReadableStorage,)*> ReadableStorage for ($($T,)*) {
			type Read = ($($T::Read,)*);
			#[inline(always)]
			fn get(world_storage: &Components, archetype_storage: &Components) -> Option<Self::Read> {
				$(let $T = $T::get(world_storage, archetype_storage)?;)*
				Some(($($T,)*))
			}
		}
	};
}

tuple!();
tuple!(T0);
tuple!(T0, T1);
tuple!(T0, T1, T2);
tuple!(T0, T1, T2, T3);

/// BorrowedStorageMut ///

impl BorrowedStorageMut for () {
    type ItemMut = ();
    type BatchMut = ();
    fn write(&mut self, _index: usize, _item: Self::ItemMut) {
    }
    fn write_batch(&mut self) -> Self::BatchMut {}
}

impl<T0: BorrowedStorageMut> BorrowedStorageMut for (T0,) {
    type ItemMut = (T0::ItemMut,);
    type BatchMut = (T0::BatchMut,);
    fn write(&mut self, index: usize, item: Self::ItemMut) {
        self.0.write(index, item.0);
    }
    fn write_batch(&mut self) -> Self::BatchMut {
        (self.0.write_batch(),)
    }
}

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

