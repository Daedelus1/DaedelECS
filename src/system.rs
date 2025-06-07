use daedelecs_generators::generate_component_tuple_impls;
use crate::entity::Entity;
use crate::world::World;
use crate::Component;
use std::any::TypeId;

#[allow(dead_code)]
pub trait System<State> where Self: 'static {
    type Data: SystemData;

    fn run(entity: &mut Entity, world: &mut World<State>);

    fn get_required_types() -> Vec<TypeId> {
        Self::Data::type_ids()
    }

    fn entity_is_eligible(entity: &Entity, world: &World<State>) -> bool{
        world.systems_to_required_types_registry.get(&TypeId::of::<Self>()).unwrap()
            .iter()
            .all(|type_id| entity.components.contains_key(type_id))
    }
}
pub trait SystemData {
    fn type_ids() -> Vec<TypeId>;
}


generate_component_tuple_impls!(128);

