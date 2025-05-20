use bevy::prelude::*;
use common::state::{AppState, VideoState};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Video(VideoState::Sierra)), setup)
        .add_systems(OnEnter(AppState::Video(VideoState::Troika)), setup)
        .add_systems(OnEnter(AppState::Video(VideoState::Teaser)), setup);
}

fn setup(current_state: Res<State<AppState>>, mut next_state: ResMut<NextState<AppState>>) {
    let next = match current_state.get() {
        AppState::Video(VideoState::Sierra) => AppState::Video(VideoState::Troika),
        AppState::Video(VideoState::Troika) => AppState::Loading,
        AppState::Video(VideoState::Teaser) => AppState::MainMenu,
        _ => panic!("video crate shouldn't decide over non_video states"),
    };

    next_state.set(next);
}
