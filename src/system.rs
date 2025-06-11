use crate::entity::Entity;
use crate::world::World;
use crate::Component;
use daedelecs_generators::generate_component_tuple_impls;
use std::any::TypeId;

pub trait System<State>
where
    Self: 'static,
{
    type Data: SystemData;

    /// Runs an arbitrary function over all entities with all the components in SystemData.
    fn run(entity: &mut Entity, world: &mut World<State>);
}
pub trait SystemData {
    fn type_ids() -> Vec<TypeId>;
}
pub(crate) trait SystemInfo<State> {
    fn entity_is_eligible(entity: &Entity, world: &World<State>) -> bool;
    fn get_required_types() -> Vec<TypeId>;
}
impl<S: System<State>, State> SystemInfo<State> for S {
    fn entity_is_eligible(entity: &Entity, world: &World<State>) -> bool {
        world
            .systems_to_required_types_registry
            .get(&TypeId::of::<S>())
            .unwrap()
            .iter()
            .all(|type_id| entity.components.contains_key(type_id))
    }
    fn get_required_types() -> Vec<TypeId> {
        S::Data::type_ids()
    }
}
// Generates the implementation of SystemData trait for all tuples up to an arbitrarily large number of
// items. If you are depending on over 128 traits in a single system, please consider breaking
// up responsibility of your system.
generate_component_tuple_impls!(128);
