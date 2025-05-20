use bevy::{
    asset::{AssetLoader, LoadContext},
    image::{Image, TextureAtlasLayout},
};
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

#[derive(Default)]
pub(crate) struct ArtImageLoader;

impl AssetLoader for ArtImageLoader {
    type Asset = Image;
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
        let art = Art::from_buffer(&bytes)?;
        Ok(art.to_image()?)
    }

    fn extensions(&self) -> &[&str] {
        &["art", "ART"]
    }
}

#[derive(Default)]
pub(crate) struct ArtTextureAtlasLayoutLoader;

impl AssetLoader for ArtTextureAtlasLayoutLoader {
    type Asset = TextureAtlasLayout;
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
        let art = Art::from_buffer(&bytes)?;
        Ok(art.to_texture_atlas())
    }

    fn extensions(&self) -> &[&str] {
        &["art", "ART"]
    }
}
