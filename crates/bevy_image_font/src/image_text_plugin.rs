use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    platform::collections::{HashMap, HashSet},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use image::{GenericImage, GenericImageView, ImageBuffer, Rgba};

use crate::{ImageFont, ImageText, ImageTextFont, image_font_loader::ImageFontLoader};

pub struct ImageTextPlugin;

impl Plugin for ImageTextPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ImageFont>()
            .init_asset_loader::<ImageFontLoader>()
            .register_type::<ImageTextFont>()
            .register_type::<ImageText>()
            .add_systems(PostUpdate, (sync, render_text_to_image_node).chain());
    }
}

#[derive(Debug)]
pub enum ImageFontRenderError {
    MissingImageFontAsset,
    MissingTextureAsset,
    MissingLayoutAsset,
}

fn sync(
    mut events: EventReader<AssetEvent<ImageFont>>,
    mut query: Query<(&mut ImageText, &ImageTextFont)>,
) {
    let mut changed_fonts = HashSet::new();
    for id in events.read().filter_map(extract_asset_id) {
        info!("Image font {id} finished loading; marking as dirty");
        changed_fonts.insert(id);
    }

    for (mut image_text, text_font) in &mut query {
        if changed_fonts.contains(&text_font.font.id()) {
            image_text.set_changed();
        }
    }
}

fn extract_asset_id(event: &AssetEvent<ImageFont>) -> Option<AssetId<ImageFont>> {
    match *event {
        AssetEvent::Modified { id } | AssetEvent::LoadedWithDependencies { id } => Some(id),
        _ => None,
    }
}

const STRING: &str = "  !\"#$%&'()*+.-,/0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

fn render_text_to_image_node(
    image_fonts: Res<Assets<ImageFont>>,
    mut images: ResMut<Assets<Image>>,
    texture_atlas_layouts: Res<Assets<TextureAtlasLayout>>,
    mut query: Query<(&ImageText, &ImageTextFont, &mut ImageNode), Changed<ImageText>>,
) {
    let mut char_to_coord = HashMap::new();
    for (index, char) in STRING.chars().enumerate() {
        char_to_coord.insert(char, index);
    }
    for (image_text, image_text_font, mut image_node) in &mut query {
        // fetch assets
        let (_image_font, font_spritesheet, texture_atlas_layout) = match fetch_assets(
            &image_text_font,
            &image_fonts,
            &images,
            &texture_atlas_layouts,
        ) {
            Ok(t) => t,
            Err(err) => {
                error!("{:?}", err);
                return;
            }
        };

        // filter string to available chars
        let filtered = image_text
            .0
            .chars()
            .filter(|character| char_to_coord.contains_key(character));

        // find length of string
        let mut width = 0;
        let mut height = 0;
        for character in filtered {
            let index = char_to_coord
                .get(&character)
                .expect("character filtering guarantees valid characters");
            let rect = texture_atlas_layout.textures[*index];
            width += rect.width();
            height = height.max(rect.height());
        }
        info!("string should be {} by {}", width, height);

        let mut output_image = image::RgbaImage::new(width, height);
        let data = font_spritesheet.data.as_ref().unwrap();
        let font_texture: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(
            font_spritesheet.width(),
            font_spritesheet.height(),
            data.as_slice(),
        )
        .unwrap();

        let filtered = image_text
            .0
            .chars()
            .filter(|character| char_to_coord.contains_key(character));

        let mut x_pos = 0;
        for character in filtered {
            let index = char_to_coord
                .get(&character)
                .expect("character filtering guarantees valid characters");
            let rect = texture_atlas_layout.textures[*index];
            let view = font_texture.view(rect.min.x, rect.min.y, rect.width(), rect.height());
            if let Err(err) = output_image.copy_from(&*view, x_pos, 0) {
                error!("{:?}", err);
                return;
            }
            x_pos += rect.width();
        }

        let mut bevy_image = Image::new(
            Extent3d {
                width: output_image.width(),
                height: output_image.height(),
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            output_image.into_vec(),
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD,
        );
        bevy_image.sampler = ImageSampler::nearest();

        let image_handle = images.add(bevy_image);
        image_node.image = image_handle;
    }
}

fn fetch_assets<'assets>(
    text_font: &ImageTextFont,
    image_fonts: &'assets Assets<ImageFont>,
    images: &'assets Assets<Image>,
    texture_atlas_layouts: &'assets Assets<TextureAtlasLayout>,
) -> Result<
    (
        &'assets ImageFont,
        &'assets Image,
        &'assets TextureAtlasLayout,
    ),
    ImageFontRenderError,
> {
    let Some(image_font) = image_fonts.get(&text_font.font) else {
        return Err(ImageFontRenderError::MissingImageFontAsset);
    };
    let Some(image) = images.get(&image_font.image) else {
        return Err(ImageFontRenderError::MissingTextureAsset);
    };
    let Some(texture_atlas_layout) = texture_atlas_layouts.get(&image_font.texture_atlas_layout)
    else {
        return Err(ImageFontRenderError::MissingLayoutAsset);
    };
    Ok((image_font, image, texture_atlas_layout))
}
