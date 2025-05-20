use bevy::prelude::*;

#[derive(Component)]
#[require(Node)]
pub struct ImageText(pub String);

impl ImageText {
    pub fn new(text: impl Into<String>) -> Self {
        Self(text.into())
    }
}