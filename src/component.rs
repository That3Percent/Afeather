use crate::*;

pub trait Component: 'static {
    type Storage: ReadableStorage + AnyStorage;
}

impl<R: ReadableStorage + AnyStorage, T: Component<Storage = R>> ReadableStorage for T {
    type Read = R::Read;
    #[inline(always)]
    fn get(world_storage: &Components, archetype_storage: &Components) -> Option<Self::Read> {
        R::get(world_storage, archetype_storage)
    }
}

impl<W: WritableStorage + AnyStorage, T: Component<Storage = W>> WritableStorage for T {
    type ReadMut = W::ReadMut;
    fn get_mut(archetype: &Archetype) -> Option<Self::ReadMut> {
        W::get_mut(archetype)
    }
}
