use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
};
use thiserror::Error;

use crate::{Mes, mes::MesError};

#[derive(Default)]
pub struct MesLoader;

#[derive(Debug, Error)]
pub enum MesLoaderError {
    #[error("Could not read file")]
    Io(#[from] std::io::Error),
    #[error("Could not decode file")]
    Format(#[from] MesError),
}

impl AssetLoader for MesLoader {
    type Asset = Mes;
    type Settings = ();
    type Error = MesLoaderError;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        _load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut contents = String::new();
        reader.read_to_string(&mut contents).await?;
        Ok(Mes::from_contents(&contents)?)
    }

    fn extensions(&self) -> &[&str] {
        &["mes"]
    }
}
