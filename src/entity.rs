use crate::entity_builder::EntityBuilder;
use crate::Component;
use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

pub struct Entity {
    pub(crate) components: HashMap<TypeId, Rc<RefCell<dyn Component>>>,
    pub(crate) identifier: usize
}

impl Entity {

    /// Creates an `EntityBuilder` to construct an Entity.
    pub fn builder() -> EntityBuilder {
        EntityBuilder::new()
    }
    /// Attempts to get a reference to a component of a specified type from an entity.
    /// Will Return `None` if no such component is associated with the entity.
    /// Will Return `Some(&Entity)` if it does exist.
    /// Will never panic.
    pub fn get_component<C: Component>(&self) -> Option<Ref<C>> {
        if let Some(component_reference) = self.components.get(&TypeId::of::<C>()) {
            let borrowed = component_reference.borrow();
            if (borrowed.deref() as &dyn Any).is::<C>() {
                Some(Ref::map(borrowed, |comp| {
                    (comp as &dyn Any).downcast_ref::<C>().expect(&format!(
                        "Entity must have Component {:?}",
                        TypeId::of::<C>()
                    ))
                }))
            } else {
                None
            }
        } else {
            None
        }
    }
    /// Attempts to get a mutable reference to a component of a specified type from an entity.
    /// Will Return `None` if no such component is associated with the entity.
    /// Will Return `Some(&mut Entity)` if it does exist.
    /// Will never panic.
    pub fn get_mut_component<C: Component>(&mut self) -> Option<RefMut<C>> {
        if let Some(component_reference) = self.components.get_mut(&TypeId::of::<C>()) {
            let borrowed = component_reference.borrow_mut();
            if (borrowed.deref() as &dyn Any).is::<C>() {
                Some(RefMut::map(borrowed, |comp| {
                    (comp as &mut dyn Any).downcast_mut::<C>().expect(&format!(
                        "Trait must have Component {:?}",
                        TypeId::of::<C>()
                    ))
                }))
            } else {
                None
            }
        } else {
            None
        }
    }
    pub(crate) fn add_component<C: Component>(&mut self, component: C) {
        if let Some(_value) = self
            .components
            .insert(TypeId::of::<C>(), Rc::new(RefCell::new(component)))
        {
            panic!("Attempted to Overwrite a Component with another of the same type.");
        }
    }
}
impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}