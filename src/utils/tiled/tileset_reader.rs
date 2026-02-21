use bevy::prelude::*;
use bevy_ecs_tiled::prelude::tiled::{Error, Result, Tileset};
use bevy_ecs_tiled::prelude::*;
use bevy_spritesheet_animation::prelude::{Animation, Spritesheet, SpritesheetAnimation};
use std::sync::Arc;

pub fn read_sprite_animation_from_tileset(
    tileset_name: String, // tank
    tile_id: u32,         // 0
    tiled_map_assets: Res<Assets<TiledMapAsset>>,
    mut animations: ResMut<Assets<Animation>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    images: ResMut<Assets<Image>>,
) -> Option<(Sprite, SpritesheetAnimation)> {
    if let Some((tileset, handle, rows)) = read_tileset_data(tileset_name, tiled_map_assets) {
        let tile = tileset.get_tile(tile_id)?;

        let Some(animation) = &tile.animation else {
            return None;
        };

        let spritesheet = Spritesheet::new(&handle, tileset.columns as usize, rows as usize);
        let sprite = spritesheet
            .with_loaded_image(&images)
            .expect("")
            .sprite(&mut atlas_layouts);
        let mut animation_builder = spritesheet.create_animation();

        for frame in animation {
            animation_builder = animation_builder
                .add_indices([frame.tile_id as usize])
                .set_clip_duration(
                    bevy_spritesheet_animation::prelude::AnimationDuration::PerFrame(
                        frame.duration,
                    ),
                );
        }

        let final_animation = animation_builder.build();

        let animation_handle = animations.add(final_animation);

        Some((sprite, SpritesheetAnimation::new(animation_handle)))
    } else {
        None
    }
}

fn calculate_rows(
    image: &Option<bevy_ecs_tiled::prelude::tiled::Image>,
    tile_height: u32,
    margin: u32,
    spacing: u32,
) -> Result<u32> {
    image
        .as_ref()
        .map(|image| (image.height as u32 - margin + spacing) / (tile_height + spacing))
        .ok_or_else(|| {
            Error::MalformedAttributes("No <image> nor rows attribute in <tileset>".to_string())
        })
}

pub fn read_sprite_from_tileset(
    tileset_name: String, // tank
    tile_id: u32,         // 0
    tiled_map_assets: Res<Assets<TiledMapAsset>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) -> Option<Sprite> {
    if let Some((tileset, handle, rows)) = read_tileset_data(tileset_name, tiled_map_assets) {
        // Рассчитываем количество строк в спрайтшите

        // Создаём layout для TextureAtlas
        let layout = TextureAtlasLayout::from_grid(
            UVec2::new(tileset.tile_width, tileset.tile_height),
            tileset.columns,
            rows,
            None,
            None,
        );
        let layout_handle = atlas_layouts.add(layout);

        // Создаём спрайт с указанием индекса в атласе
        let sprite = Sprite::from_atlas_image(
            handle.clone(),
            TextureAtlas {
                layout: layout_handle,
                index: tile_id as usize,
            },
        );

        Some(sprite)
    } else {
        None
    }
}

fn read_tileset_data(
    tileset_name: String,
    tiled_map_assets: Res<Assets<TiledMapAsset>>,
) -> Option<(Arc<Tileset>, Handle<Image>, u32)> {
    let mut tileset_option: Option<&Arc<Tileset>> = None;
    let mut handle_option: Option<&Handle<Image>> = None;
    for (_, asset) in tiled_map_assets.iter() {
        if asset.map.tilesets().is_empty() {
            continue;
        }

        for ts in asset.map.tilesets() {
            if ts.name == tileset_name {
                tileset_option = Some(ts);
                break;
            }
        }

        if tileset_option.is_none() {
            continue;
        }

        for (name, tileset) in asset.tilesets.iter() {
            if name.contains(tileset_name.as_str()) {
                let handlers = tileset.tilemap_texture.image_handles();
                handle_option = Some(*handlers.first().unwrap());
                break;
            }
        }

        if handle_option.is_none() {
            continue;
        }

        break;
    }

    handle_option?;

    if tileset_option.is_none() || handle_option.is_none() {
        return None;
    }

    let tileset = tileset_option.unwrap();

    let rows = calculate_rows(
        &tileset.image,
        tileset.tile_height,
        tileset.margin,
        tileset.spacing,
    )
    .unwrap();

    Some((tileset.clone(), handle_option.unwrap().clone(), rows))
}
