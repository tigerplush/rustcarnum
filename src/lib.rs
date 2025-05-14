use bevy::{asset::LoadedFolder, prelude::*};
use bevy_art::ArtPlugin;
use bevy_dat::DatPlugin;
use bevy_mes::MesPlugin;

pub struct RustcarnumPlugin;

impl Plugin for RustcarnumPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins((ArtPlugin, DatPlugin, MesPlugin))
            .add_systems(Startup, load)
            .add_systems(Update, check_2);
    }
}

#[derive(Component)]
struct FolderHandleHolder(Handle<LoadedFolder>);

fn load(asset_server: Res<AssetServer>, mut commands: Commands) {
    let folder_handle = asset_server.load_folder(".");
    commands.spawn(Camera2d);
    commands.spawn(FolderHandleHolder(folder_handle));
}

fn check_2(
    mut events: EventReader<AssetEvent<LoadedFolder>>,
    handle_holder: Single<(Entity, &FolderHandleHolder)>,
    folders: Res<Assets<LoadedFolder>>,
) {
    let (entity, folder_handle) = handle_holder.into_inner();
    for event in events.read() {
        if event.is_loaded_with_dependencies(&folder_handle.0) {
            info!("loaded all");
        }
    }
}
