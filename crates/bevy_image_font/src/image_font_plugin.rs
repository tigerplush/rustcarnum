use bevy::prelude::*;

use crate::{image_font_loader::ImageFontLoader, ImageFont};


pub struct ImageFontPlugin;

impl Plugin for ImageFontPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ImageFont>().init_asset_loader::<ImageFontLoader>();
    }
}