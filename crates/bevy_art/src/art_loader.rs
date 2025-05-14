use bevy::asset::{AssetLoader, LoadContext};
use thiserror::Error;

use crate::{Art, ArtError};

#[derive(Default)]
pub(crate) struct ArtLoader;

#[derive(Debug, Error)]
pub enum ArtLoaderError {
    #[error("Could not read file")]
    Io(#[from] std::io::Error),
    #[error("Could not decode file")]
    Format(#[from] ArtError),
}

impl AssetLoader for ArtLoader {
    type Asset = Art;
    type Settings = ();
    type Error = ArtLoaderError;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        Ok(Art::from_buffer(&bytes)?)
    }

    fn extensions(&self) -> &[&str] {
        &["art", "ART"]
    }
}
