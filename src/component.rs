use std::any::Any;

/// This trait is required to act as a Component in the ECS. 
/// This trait also implements Any meaning all stored references must have the lifetime of
/// `'static`. Use `#[derive(Component)]` to easily implement this trait.
pub trait Component: Any {}