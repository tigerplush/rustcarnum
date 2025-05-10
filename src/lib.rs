use bevy::prelude::*;

pub struct RustcarnumPlugin;

impl Plugin for RustcarnumPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
    }
}
