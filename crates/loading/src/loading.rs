use bevy::prelude::*;
use bevy_mes::Mes;
use common::state::AppState;
use dat_repo::{Dat, DatRepo, MesCritterType, MesFileType, Name, Portrait};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Loading), setup)
        .add_systems(Update, load_repo.run_if(in_state(AppState::Loading)));
}

fn setup(mut repo: ResMut<DatRepo>, asset_server: Res<AssetServer>, mut commands: Commands) {
    info!("loading now");
    let path = repo.load_file_directly("art\\splash\\Splash1.bmp").unwrap();
    commands.spawn((
        ImageNode {
            image: asset_server.load(path),
            ..default()
        },
        Node {
            width: Val::Percent(100.),
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        },
        StateScoped(AppState::Loading),
    ));

    let mes = [
        ("mes\\description.mes", MesFileType::Description),
        ("mes\\item_effect.mes", MesFileType::ItemEffect),
        (
            "rules\\xp_critter.mes",
            MesFileType::Critter(MesCritterType::Xp),
        ),
        (
            "mes\\critter.mes",
            MesFileType::Critter(MesCritterType::Base),
        ),
        (
            "art\\scenery\\scenery.mes",
            MesFileType::Name(Name::Scenery),
        ),
        (
            "art\\interface\\interface.mes",
            MesFileType::Name(Name::Interface),
        ),
        (
            "art\\unique_npc\\unique_npc.mes",
            MesFileType::Name(Name::UniqueNpc),
        ),
        (
            "art\\monster\\monster.mes",
            MesFileType::Name(Name::Monster),
        ),
        (
            "art\\eye_candy\\eye_candy.mes",
            MesFileType::Name(Name::EyeCandy),
        ),
        (
            "art\\container\\container.mes",
            MesFileType::Name(Name::Container),
        ),
        ("art\\light\\light.mes", MesFileType::Name(Name::Light)),
        ("art\\tile\\tilename.mes", MesFileType::Name(Name::Tile)),
        ("art\\roof\\roofname.mes", MesFileType::Name(Name::Roof)),
        ("art\\wall\\wallname.mes", MesFileType::Name(Name::Wall)),
        (
            "art\\wall\\wallproto.mes",
            MesFileType::Name(Name::WallProto),
        ),
        (
            "art\\structure\\structure.mes",
            MesFileType::Name(Name::Structure),
        ),
        (
            "portrait\\gameport.mes",
            MesFileType::Portrait(Portrait::Game),
        ),
        (
            "portrait\\userport.mes",
            MesFileType::Portrait(Portrait::User),
        ),
    ];
    for (pattern, key) in mes {
        repo.add_mes_file_to_load(key, pattern);
    }
    info!("loading on enter done");
}

fn load_repo(
    mut dat_repo: ResMut<DatRepo>,
    mes: Res<Assets<Mes>>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if dat_repo.load_next(&asset_server) {
        return;
    }

    if !dat_repo
        .mes_handles
        .iter()
        .all(|(_, handle)| asset_server.is_loaded(handle))
    {
        return;
    }
    while let Some((mes_type, handle)) = dat_repo.mes_handles.pop() {
        if let Some(mes) = mes.get(&handle) {
            dat_repo.insert_mes(mes_type, mes);
        }
    }
    info!("done");
    next_state.set(AppState::MainMenu);
}
