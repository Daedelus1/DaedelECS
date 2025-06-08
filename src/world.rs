use crate::entity::Entity;
use crate::system::System;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct World<S> {
    pub(crate) entities: Vec<Rc<RefCell<Entity>>>,
    pub(crate) systems_to_required_types_registry: HashMap<TypeId, HashSet<TypeId>>,
    pub(crate) system_to_entities_registry: HashMap<TypeId, Vec<Rc<RefCell<Entity>>>>,
    pub state: S,
}
impl<State> World<State> {
    /// Creates a new world, with an empty registry and no entities.
    pub fn new(state: State) -> Self {
        World {
            entities: Default::default(),
            systems_to_required_types_registry: Default::default(),
            system_to_entities_registry: Default::default(),
            state,
        }
    }
    /// Registers the System as a possible system for an entity to have.
    /// Call this method once ideally as soon as the world is initialized.
    /// Will have no effect if called multiple times with the same system.  
    /// Will not panic. 
    pub fn register_system<S: System<State> + 'static>(&mut self) {
        if let Some (_) = self.systems_to_required_types_registry.insert(
            TypeId::of::<S>(),
            S::get_required_types()
                .iter()
                .map(|t| *t)
                .collect::<HashSet<TypeId>>(),
        ) {
            return;
        }
        if self.entities.is_empty() {
            return;
        }
        let mut set = Vec::new();
        self.entities
            .iter()
            .filter(|entity| S::entity_is_eligible(entity.borrow().deref(), &self))
            .for_each(|entity| {
                set.push(entity.clone());
            });
        self.system_to_entities_registry
            .insert(TypeId::of::<S>(), set);
    }
    /// Runs the indicated system over all entities with the requisite components.
    /// Will not panic unless the user-defined systems can panic.
    pub fn run_system<S: System<State> + 'static>(&mut self) {
        if let Some(entities) = self.system_to_entities_registry.get(&TypeId::of::<S>()) {
            let mut cloned_entities = Vec::with_capacity(entities.len());
            entities.clone_into(&mut cloned_entities);
            cloned_entities
                .iter()
                .for_each(|entity| S::run(entity.borrow_mut().deref_mut(), self))
        } else {
            self.register_system::<S>();
            self.run_system::<S>();
        }
    }
    pub(crate) fn add_entity(&mut self, entity: Entity) {
        let entity_rc = Rc::new(RefCell::new(entity));
        let _ = self
            .systems_to_required_types_registry
            .iter()
            .filter(|(_system, required_types)| {
                required_types
                    .iter()
                    .all(|type_id| entity_rc.borrow().components.contains_key(type_id))
            })
            .for_each(|(system, _)| {
                self.system_to_entities_registry
                    .get_mut(system)
                    .unwrap()
                    .push(entity_rc.clone());
            });
        self.entities.push(entity_rc);
    }
}
