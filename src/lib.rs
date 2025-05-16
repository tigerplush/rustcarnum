use std::path::Path;

use bevy::{asset::io::memory::Dir, prelude::*};
use bevy_art::ArtPlugin;
use bevy_dat::{Dat, DatPlugin};
use bevy_mes::MesPlugin;

pub struct RustcarnumPlugin;

impl Plugin for RustcarnumPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (800., 800.).into(),
                title: "Arcanum".into(),
                name: Some("Arcanum".into()),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((ArtPlugin, DatPlugin, MesPlugin))
        .add_systems(Startup, load)
        .add_systems(Update, build_repo);
    }
}

#[derive(Component, Deref)]
struct DatFiles(Vec<Handle<Dat>>);

fn load(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(DatFiles(vec![
        asset_server.load("tig.dat"),
        asset_server.load("arcanum1.dat"),
        asset_server.load("arcanum2.dat"),
        asset_server.load("arcanum3.dat"),
        asset_server.load("Arcanum4.dat"),
    ]));
}

#[derive(Component, Default)]
struct DatRepo {
    dir: Dir,
}

impl DatRepo {
    fn add(&mut self, dat: &Dat) {
        for entry in dat.entries() {
            self.dir.insert_asset(Path::new(&entry.filename), dat.bytes(entry));
        }
    }
}

fn build_repo(dat_files: Single<(Entity, &DatFiles)>, asset_server: Res<AssetServer>, dats: Res<Assets<Dat>>, mut commands: Commands) {
    let (entity, handles) = dat_files.into_inner();
    if handles.iter().all(|handle| asset_server.is_loaded(handle)) {
        let mut repo = DatRepo::default();
        for handle in &handles.0 {
            if let Some(dat) = dats.get(handle) {
                repo.add(dat);
            }
        }
        commands.spawn(repo);
        commands.entity(entity).despawn();
        info!("repo done");
    }
}
