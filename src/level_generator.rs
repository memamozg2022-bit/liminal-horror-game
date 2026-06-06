use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use crate::texture_generator::TextureGenerator;

pub struct LevelGenerator {
    texture_gen: TextureGenerator,
}

impl LevelGenerator {
    pub fn new() -> Self {
        LevelGenerator {
            texture_gen: TextureGenerator::new(),
        }
    }

    pub fn generate_chunk_texture(&mut self, chunk_x: i32, chunk_y: i32, images: &mut ResMut<Assets<Image>>) -> Handle<Image> {
        let size = 256usize;
        let mut final_data = vec![0u8; size * size * 4];

        let base = self.texture_gen.generate_liminal_texture(chunk_x, chunk_y);
        let details = self.texture_gen.generate_creepy_details(chunk_x, chunk_y);
        let hazards = self.texture_gen.generate_hazard_zones(chunk_x, chunk_y);

        for i in 0..size * size {
            let base_idx = i * 4;

            let mut r = base[base_idx] as f32;
            let mut g = base[base_idx + 1] as f32;
            let mut b = base[base_idx + 2] as f32;

            if details[base_idx + 3] > 0 {
                let alpha = details[base_idx + 3] as f32 / 255.0;
                r = r * (1.0 - alpha) + details[base_idx] as f32 * alpha;
                g = g * (1.0 - alpha) + details[base_idx + 1] as f32 * alpha;
                b = b * (1.0 - alpha) + details[base_idx + 2] as f32 * alpha;
            }

            if hazards[base_idx + 3] > 0 {
                let alpha = hazards[base_idx + 3] as f32 / 255.0;
                r = r * (1.0 - alpha) + hazards[base_idx] as f32 * alpha;
                g = g * (1.0 - alpha) + hazards[base_idx + 1] as f32 * alpha;
                b = b * (1.0 - alpha) + hazards[base_idx + 2] as f32 * alpha;
            }

            final_data[base_idx] = (r.max(0.0).min(255.0)) as u8;
            final_data[base_idx + 1] = (g.max(0.0).min(255.0)) as u8;
            final_data[base_idx + 2] = (b.max(0.0).min(255.0)) as u8;
            final_data[base_idx + 3] = 255;
        }

        images.add(Image::new(
            Extent3d {
                width: size as u32,
                height: size as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            final_data,
            TextureFormat::Rgba8UnormSrgb,
        ))
    }
}
