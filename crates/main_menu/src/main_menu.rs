use bevy::prelude::*;
use bevy_image_font::{ImageFont, ImageText, ImageTextFont};
use common::state::AppState;
use dat_repo::DatRepo;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::MainMenu), setup);
}

fn setup(dat_repo: Res<DatRepo>, asset_server: Res<AssetServer>, mut commands: Commands) {
    info!("main menu");
    let path = dat_repo.load_file_by_num(329).unwrap();
    let font = dat_repo.load_file_by_num(327).unwrap();
    commands
        .spawn((
            ImageNode {
                image: asset_server.load(path),
                ..default()
            },
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            StateScoped(AppState::MainMenu),
        ))
        .with_children(|parent| {
            // parent.spawn((ImageNode {
            //     image: asset_server.load(&font),
            //     ..default()
            // },
            //     Node {
            //         align_self: AlignSelf::Center,
            //         justify_self: JustifySelf::Center,
            //         ..default()
            //     },));
            parent.spawn((
                ImageText::new("Hallo Nico"),
                ImageTextFont {
                    font: asset_server.load(&font),
                },
                Node {
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    ..default()
                },
            ));
        });
}
