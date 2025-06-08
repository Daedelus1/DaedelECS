use crate::component::Component;
use crate::entity::Entity;
use crate::world::World;

pub struct EntityBuilder {
    entity: Entity,
}
impl EntityBuilder {
    pub(crate) fn new() -> Self {
        EntityBuilder {
            entity: Entity {
                components: Default::default(),
                identifier: Default::default(),
            },
        }
    }
    /// Attaches a component to the entity being built.
    /// Will panic if the component already exists for the entity
    pub fn with<C: Component>(mut self, component: C) -> Self {
        self.entity.add_component(component);
        self
    }
    /// Attaches the entity to the ECS System.
    /// Unlike other ECS's, there is no built-in means of identification,
    /// so you cannot address the entity individually without a unique component.
    pub fn add_to_world<State>(mut self, world: &mut World<State>) {
        self.entity.identifier = world.get_next_identifier();
        world.add_entity(self.entity);
    }
}
