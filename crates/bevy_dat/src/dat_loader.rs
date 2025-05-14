use bevy::{asset::AssetLoader, prelude::*};
use thiserror::Error;

use crate::dat::{Dat, DatError};

#[derive(Default)]
pub(crate) struct DatLoader;

#[derive(Debug, Error)]
pub enum DatLoaderError {
    #[error("Could not read file")]
    Io(#[from] std::io::Error),
    #[error("Could not decode file")]
    Format(#[from] DatError),
}

impl AssetLoader for DatLoader {
    type Asset = Dat;
    type Settings = ();
    type Error = DatLoaderError;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        _load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        Ok(Dat::from_buffer(&bytes)?)
    }

    fn extensions(&self) -> &[&str] {
        &["dat"]
    }
}
