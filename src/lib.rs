use bevy::prelude::*;
use bevy_art::*;

pub struct RustcarnumPlugin;

impl Plugin for RustcarnumPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(ArtPlugin)
            .add_systems(Startup, load)
            .add_systems(Update, check);
    }
}

#[derive(Component)]
struct ArtHandleHolder(Handle<Art>);

fn load(asset_server: Res<AssetServer>, mut commands: Commands) {
    let handle: Handle<Art> = asset_server.load("dfmbnsad.ART");
    commands.spawn(ArtHandleHolder(handle));
    commands.spawn(Camera2d);
}

fn check(
    handle_holder: Single<(Entity, &ArtHandleHolder)>,
    art: Res<Assets<Art>>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    let (entity, art_handle) = handle_holder.into_inner();
    if let Some(specific_art) = art.get(&art_handle.0) {
        commands.spawn(ImageNode {
            image: images.add(specific_art.to_image().unwrap()),
            ..default()
        });
        commands.entity(entity).despawn();
    }
}
