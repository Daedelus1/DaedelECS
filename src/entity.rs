use crate::entity_builder::EntityBuilder;
use crate::Component;
use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

#[allow(dead_code)]
pub(crate) struct Entity {
    pub(crate) components: HashMap<TypeId, Rc<RefCell<dyn Component>>>,
}

impl Entity {
    #[allow(dead_code)]
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
    pub(crate) fn new() -> Self {
        Entity {
            components: Default::default(),
        }
    }
    pub(crate) fn add_component<C: Component>(&mut self, component: C) {
        if let Some(_value) = self.components
            .insert(TypeId::of::<C>(), Rc::new(RefCell::new(component))) {
            println!("Component Overwritten!");
        }
    }
    pub fn builder() -> EntityBuilder {
        EntityBuilder::new()
    }
}