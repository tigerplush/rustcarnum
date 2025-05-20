use bevy::state::state::States;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum AppState {
    #[default]
    Preload,
    Video(VideoState),
    Loading,
    MainMenu,
}

impl AppState {
    pub fn first_video() -> AppState {
        AppState::Video(VideoState::Sierra)
    }

    pub fn teaser_video() -> AppState {
        AppState::Video(VideoState::Teaser)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, States)]
pub enum VideoState {
    Sierra,
    Troika,
    Teaser,
}
