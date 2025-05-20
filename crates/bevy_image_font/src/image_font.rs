use bevy::prelude::*;

#[derive(Component)]
pub struct ImageTextFont {
    pub font: Handle<ImageFont>,
}

#[derive(Asset, TypePath)]
pub struct ImageFont {
    pub(crate) image: Handle<Image>,
    pub(crate) texture_atlas_layout: Handle<TextureAtlasLayout>,
}