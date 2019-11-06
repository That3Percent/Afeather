use super::*;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Copy, Debug, Clone, Hash)]
pub struct UniqueId(pub u128);

impl Component for UniqueId {
    type Storage = PerEntity<Self>;
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct EntitySlot {
    archetype_index: usize,
    entity_index: usize,
}

// TODO: Rather than hiding methods on the world, wrap it in some kind of processor that has the schedule_query, schedule_update, etc methods.

// TODO: Update / Version rules
// 0. The World starts at Version(0)
// 1. An update always runs.
// 2. When an update runs, the world version number is increased by 1, and all components modified by add or mutate (but not delete) have their version set to this.
// 3. When a query runs, it necessarily reads components.
// 4. If a processed component is read and it's version is out of date, it runs the process to update the component.
// 5. A component requires 2 versions - UpdatedAt, and CurrentAsOf.
// 6. A process is stored on the component that it needs to update.

pub struct World {
    archetypes: Vec<Option<Archetype>>,
    entities: HashMap<UniqueId, EntitySlot>,
    globals: Components,
}

impl Default for World {
	fn default() -> Self {
		Self::new()
	}
}

impl World {
    pub fn new() -> Self {
        Self {
            archetypes: Vec::new(),
            entities: HashMap::new(),
            globals: Components::new(),
        }
    }

    pub fn entity_count(&self) -> usize {
        self.archetypes
            .iter()
            .filter_map(|a| a.as_ref().map(|a| a.num_entities()))
            .sum()
    }

    // TODO: Make this ReadableStorage, to support eg: tuples
    pub fn read_component<T: ReadableStorage>(
        &self,
        entity: &UniqueId,
    ) -> Option<<<<T as ReadableStorage>::Read as RefLike>::Borrowed as BorrowedStorage>::Item>
    where
        <<<T as ReadableStorage>::Read as RefLike>::Borrowed as BorrowedStorage>::Item: Copy,
    {
        let slot = self.entities.get(entity)?;
        let archetype = &self.archetypes[slot.archetype_index].as_ref();
        let storage = T::get(&self.globals, &archetype.unwrap().components)?;
        storage.borrow().read(slot.entity_index)
    }

    fn add_entity_inner<T: EntityWriter + ArchetypeFilter + ArchetypeInitializer>(
        &mut self,
        entity: T,
    ) -> EntitySlot {
        // First try writing it to a matching archetype
        for (slot, archetype_index) in self.archetypes.iter_mut().zip(0..std::usize::MAX) {
            if let Some(archetype) = slot {
                if entity.includes(archetype) {
                    let entity_index = archetype.entity_write_slot();
                    entity.write(archetype, entity_index);
                    return EntitySlot {
                        archetype_index,
                        entity_index,
                    };
                }
            }
        }
        // If no archetype matches, create a new one.
        let mut archetype = Archetype::new();
        let entity_index = archetype.entity_write_slot();
        entity.initialize(&mut archetype);

        // Find an empty slot to place the archetype
        for (slot, archetype_index) in self.archetypes.iter_mut().zip(0..std::usize::MAX) {
            if slot.is_none() {
                *slot = Some(archetype);
                return EntitySlot {
                    archetype_index,
                    entity_index,
                };
            }
        }

        // Create a new slot if none were found.
        self.archetypes.push(Some(archetype));
        EntitySlot {
            archetype_index: self.archetypes.len() - 1,
            entity_index,
        }
    }

    pub fn add_entity<T: EntityWriter + ArchetypeFilter + ArchetypeInitializer>(
        &mut self,
        unique_id: UniqueId,
        entity: T,
    ) {
        // No duplicate entries
        assert!(!self.entities.contains_key(&unique_id));
        // Extend entity with unique_id component
        let entity = (unique_id, entity);
        let slot = self.add_entity_inner(entity);
        self.entities.insert(unique_id, slot);
    }

	pub fn remove_entity(&mut self, unique_id: UniqueId) {
		let slot = self.entities.remove(&unique_id);
		match slot {
			Some(slot) => {
				let archetype = &mut self.archetypes[slot.archetype_index];
				match archetype {
					Some(inner) => {
						inner.remove_entity(slot.entity_index);
						if inner.num_entities() == 0 {
							*archetype = None;
						}
					},
					None => unreachable!(),
				}
			},
			None => {
				#[cfg(debug_assertions)]
				unreachable!();
			}
		}
	}

    pub fn execute_query<T: Query>(&self, query: &T) -> T::Output {
        let query_data = QueryData::new(&self.globals, &self.archetypes);
        query.execute(query_data)
    }

    pub fn execute_update<T: Update>(&mut self, update: &T) {
        update.execute(&self.globals, self.archetypes.iter_mut())
    }

    pub fn execute_process<T: Process>(&mut self, process: &T) {
        for archetype in self.archetypes.iter_mut() {
            if let Some(archetype) = archetype {
                // TODO: Just add an archetype iterator.
                if let Some(read) = T::Reads::get(&self.globals, &archetype.components) {
                    if let Some(write) = T::Writes::get_mut(archetype) {
                        let read_borrow = read.borrow();
                        let mut write_borrow = write.borrow_mut();
                        let read_batch = read_borrow.read_batch();
                        let write_batch = write_borrow.write_batch();
                        process.execute(read_batch, write_batch);
                    }
                }
            }
        }
    }
}
