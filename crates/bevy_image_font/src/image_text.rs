use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(ImageNode)]
pub struct ImageText(pub String);

impl ImageText {
    pub fn new(text: impl Into<String>) -> Self {
        Self(text.into())
    }
}
