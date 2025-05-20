use bevy::asset::AssetLoader;
use bevy_art::{Art, ArtError};
use thiserror::Error;

use crate::ImageFont;

#[derive(Default)]
pub(crate) struct ImageFontLoader;

#[derive(Debug, Error)]
pub(crate) enum ImageFontLoaderError {
    #[error("Could not read file")]
    Io(#[from] std::io::Error),
    #[error("Could not decode file")]
    Format(#[from] ArtError),
}

impl AssetLoader for ImageFontLoader {
    type Asset = ImageFont;
    type Settings = ();
    type Error = ImageFontLoaderError;

    async fn load(
            &self,
            reader: &mut dyn bevy::asset::io::Reader,
            _settings: &Self::Settings,
            load_context: &mut bevy::asset::LoadContext<'_>,
        ) -> Result<Self::Asset, Self::Error> {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let art = Art::from_buffer(&bytes)?;
            let image = art.to_image()?;
            let texture_atlas = art.to_texture_atlas();

        Ok(ImageFont {
            image: load_context.add_labeled_asset("huh".into(), image),
            texture_atlas_layout: load_context.add_labeled_asset("huh".into(), texture_atlas),
        })
    }

    fn extensions(&self) -> &[&str] {
        &["art", "ART"]
    }
}