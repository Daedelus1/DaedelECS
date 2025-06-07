use crate::component::Component;
use crate::entity::Entity;
use crate::world::World;

#[allow(dead_code)]
pub(crate) struct EntityBuilder {
    entity: Entity
}
#[allow(dead_code)]
impl EntityBuilder {
    pub fn new() -> Self {
        EntityBuilder {
            entity: Entity {
                components: Default::default()
            }
        }
    }
    pub fn add<C: Component>(mut self, component:C) -> Self{
        self.entity.add_component(component);
        self
    }
    pub fn add_to_world<State>(self, world: &mut World<State>) {
        world.add_entity(self.entity);
    }
}
