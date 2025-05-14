use bevy::prelude::*;

use crate::{mes::Mes, mes_loader::MesLoader};

pub struct MesPlugin;

impl Plugin for MesPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Mes>().init_asset_loader::<MesLoader>();
    }
}
