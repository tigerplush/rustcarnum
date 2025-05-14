use bevy::prelude::*;

use crate::{dat::Dat, dat_loader::DatLoader};

pub struct DatPlugin;

impl Plugin for DatPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Dat>().init_asset_loader::<DatLoader>();
    }
}
