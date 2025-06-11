pub mod component;
pub mod entity;
pub mod entity_builder;
pub mod system;
pub mod world;

use crate::component::Component;
use daedelecs_core::Component;


#[cfg(test)]
mod tests {
    use crate::system::SystemInfo;
use crate::component::Component;
    use crate::entity::Entity;
    use crate::system::System;
    use crate::world::World;
    use daedelecs_core::Component;
    use std::any::TypeId;
    use std::collections::HashSet;
    use std::fmt::Debug;

    #[derive(Debug, Component)]
    struct Name(String);
    #[derive(Debug, Component)]
    struct Health(i32);
    #[allow(dead_code)]
    #[derive(Debug, Component)]
    struct Strength(i32);
    struct AddHealthSystem {}
    struct PrintSystem {}
    struct PrintWorldSystem {}
    struct SapStrengthSystem {}
    #[derive(Component)]
    struct WorldEntity {}
    impl<T> System<T> for AddHealthSystem {
        type Data = Health;

        fn run(entity: &mut Entity, _world: &mut World<T>) {
            let mut health = entity.get_mut_component::<Health>().unwrap();
            health.0 += 1;
        }
    }
    impl<T> System<T> for SapStrengthSystem {
        type Data = Strength;

        fn run(entity: &mut Entity, _world: &mut World<T>) {
            let mut strength = entity.get_mut_component::<Strength>().unwrap();
            strength.0 -= 1;
        }
    }
    impl<T> System<T> for PrintSystem {
        type Data = Name;

        fn run(entity: &mut Entity, _world: &mut World<T>) {
            let name = entity.get_component::<Name>().unwrap();
            print!("Entity[name=\"{}\"", name.0);
            if let Some(health) = entity.get_component::<Health>() {
                print!(", health={}", health.0);
            }
            if let Some(strength) = entity.get_component::<Strength>() {
                print!(", strength={}", strength.0);
            }
            print!("]\n");
        }
    }
    impl<T: Debug> System<T> for PrintWorldSystem {
        type Data = WorldEntity;

        fn run(_entity: &mut Entity, world: &mut World<T>) {
            println!("{:?}", world.state);
            world.run_system::<PrintSystem>()
        }
    }
    #[test]
    fn ecs_visual_test() {
        let mut world = World::new(());

        Entity::builder()
            .with(Name("foo".to_string()))
            .with(Health(5))
            .add_to_world(&mut world);
        Entity::builder()
            .with(Name("bar".to_string()))
            .with(Health(10))
            .with(Strength(1))
            .add_to_world(&mut world);
        Entity::builder()
            .with(Name("qux".to_string()))
            .with(Strength(3))
            .add_to_world(&mut world);
        Entity::builder()
            .with(WorldEntity {})
            .add_to_world(&mut world);

        world.register_system::<AddHealthSystem>();
        world.register_system::<SapStrengthSystem>();

        assert!(!HashSet::from([TypeId::of::<Health>()]).contains(&TypeId::of::<Strength>()));

        assert!(AddHealthSystem::entity_is_eligible(&world.entities.values().find(|_| true).unwrap().borrow(), &world));
        assert!(!SapStrengthSystem::entity_is_eligible(&world.entities.values().find(|_|true).unwrap().borrow(), &world));

        world.run_system::<PrintSystem>();
        println!();
        world.run_system::<AddHealthSystem>();
        world.run_system::<PrintSystem>();
        println!();
        world.run_system::<SapStrengthSystem>();
        world.run_system::<PrintSystem>();
        world.run_system::<PrintWorldSystem>();
    }

    #[test]
    fn entity_eligibility_test() {
        let mut world = World::new(());

        Entity::builder()
            .with(Health(5))
            .add_to_world(&mut world);
        Entity::builder()
            .with(Health(10))
            .with(Strength(1))
            .add_to_world(&mut world);
        // Entity::builder()
        //     .add(Name("qux".to_string()))
        //     .add(Strength(3))
        //     .add_to_world(&mut world);

        world.register_system::<SapStrengthSystem>();
        world.register_system::<AddHealthSystem>();

        assert!(!HashSet::from([TypeId::of::<Health>()]).contains(&TypeId::of::<Strength>()));
        assert!(HashSet::from([TypeId::of::<Health>()]).contains(&TypeId::of::<Health>()));

        assert_eq!(*world.systems_to_required_types_registry.get(&TypeId::of::<AddHealthSystem>()).unwrap(),
                   HashSet::from([TypeId::of::<Health>()]));
        assert_eq!(*world.systems_to_required_types_registry.get(&TypeId::of::<SapStrengthSystem>()).unwrap(),
                   HashSet::from([TypeId::of::<Strength>()]));

        // println!("{:?}", [TypeId::of::<Strength>(), TypeId::of::<Health>()]);
        // println!("{:?}", world.entities.values().find(|_|true).unwrap().borrow().components.iter().map(|(type_id, _)| *type_id).collect::<Vec<TypeId>>());
        // println!("{:?}", AddHealthSystem::get_required_types());

        assert_eq!(TypeId::of::<Strength>(), TypeId::of::<Strength>());
        assert_eq!(TypeId::of::<Health>(), TypeId::of::<Health>());
        assert_ne!(TypeId::of::<Strength>(), TypeId::of::<Health>());

        assert_eq!(world.entities.values().find(|_|true).unwrap().borrow().components.contains_key(&TypeId::of::<Health>()), true);
        assert_eq!(world.entities.values().find(|_|true).unwrap().borrow().components.contains_key(&TypeId::of::<Strength>()), false);
        //
        let entity = world.entities.values().find(|_|true).unwrap().borrow();

        assert_eq!(entity.components.keys().map(|t| *t).collect::<Vec<TypeId>>(), vec![TypeId::of::<Health>()]);

        assert_eq!(AddHealthSystem::entity_is_eligible(&world.entities.values().find(|_|true).unwrap().borrow(), &world), true);
        assert_eq!(SapStrengthSystem::entity_is_eligible(&world.entities.values().find(|_|true).unwrap().borrow(), &world), false);
    }
    #[test]
    fn sender_minimal_failing_test() {
        let mut world = World::new(());
        #[allow(dead_code)]
        #[derive(Component, Debug)]
        struct ExampleComponent(i32);
        #[allow(dead_code)]
        #[derive(Component)]
        struct AnotherExampleComponent(i32);
        struct ExampleSystem;
        impl System<()> for ExampleSystem {
            type Data = ExampleComponent;

            fn run(entity: &mut Entity, _world: &mut World<()>) {
                let foo = entity.get_component::<ExampleComponent>().unwrap();
                println!("{foo:?}");
            }
        }
        // Entity::builder().add(AnotherExampleComponent(1)).add_to_world(&mut world);
        Entity::builder().with(ExampleComponent(2)).add_to_world(&mut world);
        // Entity::builder().add(ExampleComponent(3)).add(AnotherExampleComponent(4)).add_to_world(&mut world);
        Entity::builder().add_to_world(&mut world);
        world.run_system::<ExampleSystem>()
    }

    #[test]
    fn test() {
        #[derive(Component)]
        struct ExampleComponent {}
        struct ExampleSystem {}
        impl System<()> for ExampleSystem {
            type Data = ExampleComponent;

            fn run(_entity: &mut Entity, _world: &mut World<()>) {
                print!("Running!")
            }
        }
        let mut world = World::new(());
        Entity::builder()
            .with(ExampleComponent {})
            .add_to_world(&mut world);
        world.run_system::<ExampleSystem>();
        assert!(world.systems_to_required_types_registry.contains_key(&TypeId::of::<ExampleSystem>()));
        assert!(world.systems_to_required_types_registry.get(&TypeId::of::<ExampleSystem>()).is_some());
        assert!(
            world
                .system_to_entities_registry
                .get(&TypeId::of::<ExampleSystem>())
                .is_some()
        );
    }
}
