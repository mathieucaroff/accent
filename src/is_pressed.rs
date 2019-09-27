use crate::structure::EitherKey;

pub trait IsPressed {
    fn is_pressed(&self) -> bool;
}

impl IsPressed for EitherKey<'_, '_> {
    fn is_pressed(&self) -> bool {
        self.0.is_pressed() || self.1.is_pressed()
    }
}
