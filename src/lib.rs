use bevy::{
    asset::io::{AssetSource, AssetSourceId, memory::MemoryAssetReader},
    color::palettes::css::BLACK,
    image::ImageSamplerDescriptor,
    prelude::*,
    window::WindowMode,
};
use bevy_art::ArtPlugin;
use bevy_dat::{Dat, DatPlugin};
use bevy_image_font::ImageFontPlugin;
use bevy_mes::MesPlugin;
use common::state::AppState;
use dat_repo::DatRepo;

pub struct RustcarnumPlugin;

impl Plugin for RustcarnumPlugin {
    fn build(&self, app: &mut App) {
        let dat_repo = DatRepo::default();
        let reader = MemoryAssetReader {
            root: dat_repo.dir.clone(),
        };
        app.register_asset_source(
            AssetSourceId::from_static("memory"),
            AssetSource::build().with_reader(move || Box::new(reader.clone())),
        )
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (800., 800.).into(),
                        title: "Arcanum".into(),
                        name: Some("Arcanum".into()),
                        resizable: false,
                        // mode: WindowMode::Fullscreen(
                        //     MonitorSelection::Primary,
                        //     VideoModeSelection::Current,
                        // ),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor::nearest(),
                }),
        )
        .insert_resource(dat_repo)
        .insert_resource(ClearColor(BLACK.into()))
        .init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>()
        .add_plugins((ArtPlugin, DatPlugin, ImageFontPlugin, MesPlugin))
        .add_plugins((loading::plugin, main_menu::plugin, video::plugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            wait_for_initializing.run_if(in_state(AppState::Preload)),
        );
    }
}

#[derive(Component, Deref)]
struct DatFiles(Vec<Handle<Dat>>);

fn setup(mut dat_repo: ResMut<DatRepo>, asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(Camera2d);

    dat_repo.add_dat(asset_server.load("tig.dat"));
    dat_repo.add_dat(asset_server.load("arcanum1.dat"));
    dat_repo.add_dat(asset_server.load("arcanum2.dat"));
    dat_repo.add_dat(asset_server.load("arcanum3.dat"));
    dat_repo.add_dat(asset_server.load("Arcanum4.dat"));
    dat_repo.add_dat(asset_server.load("modules/Arcanum.dat"));
}

fn wait_for_initializing(
    mut dat_repo: ResMut<DatRepo>,
    dats: Res<Assets<Dat>>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if dat_repo.loaded_dat(&asset_server) {
        dat_repo.fill(&dats);
        next_state.set(AppState::first_video());
    }
}
