use crate::*;
use std::any::TypeId;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Components {
    pub any: HashMap<TypeId, Rc<dyn AnyStorage>>,
}

impl Components {
    pub fn new() -> Self {
        Self {
            any: HashMap::new(),
        }
    }

    pub fn get_storage<T: AnyStorage>(&self) -> Option<Rc<T>> {
        match self.any.get(&TypeId::of::<T>()) {
            Some(storage) => match storage.clone().downcast_rc() {
                Ok(r) => Some(r),
                Err(_) => unreachable!(),
            },
            None => None,
        }
    }

    pub fn get_storage_mut<T: AnyStorage>(&self) -> Option<Rc<T>> {
        match self.any.get(&TypeId::of::<T>()) {
            Some(storage) => match storage.clone().downcast_rc() {
                Ok(r) => Some(r),
                Err(_) => unreachable!(),
            },
            None => None,
        }
    }

    pub fn add<T: AnyStorage>(&mut self, storage: T) {
        let id = TypeId::of::<T>();
        assert!(
            self.any.get(&id).is_none(),
            "Added component twice to the same archetype"
        );
        self.any.insert(id, Rc::new(storage));
    }

	pub fn remove_entity(&mut self, index: usize, top: usize) {
		for storage in self.any.values_mut() {
			storage.remove_entity(index, top);
		}
	}
}
