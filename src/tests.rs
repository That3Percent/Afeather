use crate::*;
use std::collections::HashMap;

macro_rules! entities {
    ($($x:expr,)*) => (
       {
		   let mut world = World::new();
		   let mut ids = 0..std::u128::MAX;
		   $(
			   {
					let uid = UniqueId(ids.next().unwrap());
					world.add_entity(uid, $x);
			   }
		   );*
		   world
	   }
    );
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Level(usize);
impl Component for Level {
    type Storage = PerEntity<Self>;
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct SourceId(u128);
impl Component for SourceId {
    type Storage = PerArchetype<Self>;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct SamplingRate(u8);
impl Component for SamplingRate {
    type Storage = PerArchetype<Self>;
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Kind(&'static str);
impl Component for Kind {
    type Storage = PerArchetype<Self>;
}

struct EntityCountsQuery {}
impl Query for EntityCountsQuery {
    type Reads = (SourceId, UniqueId);
    type Output = HashMap<SourceId, usize>;
    fn execute(&self, data: QueryData<Self::Reads>) -> Self::Output {
        let mut result = HashMap::<SourceId, usize>::new();
        for (source_id, ids) in data {
            let current = result.entry(*source_id).or_default();
            *current += ids.len();
        }
        result
    }
}

struct CullSamplingRate(SamplingRate);

impl CullArchetypes for CullSamplingRate {
    type Reads = SamplingRate;
    fn should_cull(&self, data: &SamplingRate) -> bool {
        data >= &self.0
    }
}

struct IncreaseLevelK {}
impl Process for IncreaseLevelK {
    type Reads = Option<Kind>;
    type Writes = Level; // TODO: It should not be possible to have RW access to data.
    fn execute(&self, read: Option<&Kind>, write: &mut [Level]) {
        let add = match read {
            None => 1,
            Some(&Kind("x")) => 10,
            _ => 100,
        };
        for level in write {
            *level = Level(level.0 + add);
        }
    }
}

struct IncreaseLevel {}
impl Process for IncreaseLevel {
    type Reads = ();
    type Writes = Level;
    fn execute(&self, _read: (), write: &mut [Level]) {
        for level in write {
            *level = Level(level.0 + 1);
        }
    }
}

#[test]
fn can_add_rich_entity() {
    let world = entities! {
        (Level(1), Kind("k")),
    };

    let uid = UniqueId(0);

    assert_eq!(
        world.read_component::<(Kind, Level)>(&uid),
        Some((&Kind("k"), &Level(1)))
    );
    assert_eq!(world.read_component::<SamplingRate>(&uid), None);
}

#[test]
fn can_execute_query() {
    let world = entities! {
        SourceId(0),
        SourceId(1),
        SourceId(1),
        (SourceId(1), Kind("k")),
        SourceId(0),
        SourceId(1),
        SourceId(1),
    };

    // TODO: Query should be async. Call world.run()
    let counts = world.execute_query(&EntityCountsQuery {});
    assert_eq!(counts.len(), 2);
    assert_eq!(counts.get(&SourceId(0)), Some(&2));
    assert_eq!(counts.get(&SourceId(1)), Some(&5));
}

#[test]
fn can_execute_update() {
    let mut world = entities! {
        SamplingRate(2),
        SamplingRate(1),
        SamplingRate(3),
        SamplingRate(1),
        SamplingRate(2),
        SamplingRate(3),
    };

    // TODO: Update should be async. Call world.run()
    assert_eq!(world.entity_count(), 6);
    world.execute_update(&CullSamplingRate(SamplingRate(2)));
    assert_eq!(world.entity_count(), 2);
}

// TODO: Add sparse zst to entity and ensure it stays in the same archetype, and will initialize the storage for the archetype
// TODO: Add a sparse zst to an existing entity in an archetype which already has storage, and ensure that works as well.
// TODO: Ensure that an entity with fewer components is not written to an entity with more components (the can_write_to and can_be_written_from go both ways)
// TODO: A way to specify to initialize a derived component from a process when it's not there (perhaps the default?). Should that also automatically delete components when other components are deleted?
//       This could just be to have an update that adds and removes them.
// TODO: Write a process that removes archetypes with no components

/* TODO:
#[test]
fn can_add_component_to_entity() {
    // Move an entity from one archetype to another by...
    // Moving the entity out of the archetype as a tuple eg: remove_enity returns (entity_data...)
    // Append to the tuple
    // Move the entity back into the system.
}
*/

#[test]
fn can_write_system() {
    let mut world = entities! {
        Level(1),
        (Level(1), Kind("x")),
    };

    world.execute_process(&IncreaseLevelK {});
    assert_eq!(world.read_component::<Level>(&UniqueId(0)), Some(&Level(2)));
    assert_eq!(
        world.read_component::<Level>(&UniqueId(1)),
        Some(&Level(11))
    );

    world.execute_process(&IncreaseLevel {});
    assert_eq!(world.read_component::<Level>(&UniqueId(0)), Some(&Level(3)));
    assert_eq!(
        world.read_component::<Level>(&UniqueId(1)),
        Some(&Level(12))
    );
}
