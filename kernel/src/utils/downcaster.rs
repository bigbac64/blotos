use core::any::Any;

pub trait Downcast: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Évite la redondance d'avoir as_any et as_any_mut pour chaque structure ayant besoin de downcast
impl<T: Any> Downcast for T {
    fn as_any(&self) -> &dyn Any { self }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}