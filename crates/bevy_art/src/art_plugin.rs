use bevy::prelude::*;

use crate::{
    Art,
    art_loader::{ArtImageLoader, ArtLoader, ArtTextureAtlasLayoutLoader},
};

pub struct ArtPlugin;

impl Plugin for ArtPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Art>()
            .init_asset_loader::<ArtLoader>()
            .init_asset_loader::<ArtImageLoader>()
            .init_asset_loader::<ArtTextureAtlasLayoutLoader>();
    }
}
