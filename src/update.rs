use crate::*;

pub trait Update {
    fn execute<'a>(
        &self,
        world_storage: &Components,
        world: impl Iterator<Item = &'a mut Option<Archetype>>,
    );
}

pub trait CullArchetypes {
    type Reads: ReadableStorage;
    fn should_cull(
        &self,
        data: <<<<Self as CullArchetypes>::Reads as ReadableStorage>::Read as RefLike>::Borrowed as BorrowedStorage>::Batch,
    ) -> bool;
}

impl<T: CullArchetypes> Update for T {
    fn execute<'b>(
        &self,
        world_storage: &Components,
        archetypes: impl Iterator<Item = &'b mut Option<Archetype>>,
    ) {
        for archetype in archetypes {
            let mut f = false;
            {
                if let Some(storage) = archetype
                    .as_ref()
                    .and_then(|a| T::Reads::get(world_storage, a.components()))
                {
                    let borrow = storage.borrow();
                    if self.should_cull(borrow.read_batch()) {
                        f = true;
                    }
                }
            }
            if f {
                *archetype = None
            }
        }
    }
}
