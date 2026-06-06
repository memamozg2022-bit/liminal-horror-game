use noise::Perlin;

pub struct TextureGenerator {
    perlin: Perlin,
}

impl TextureGenerator {
    pub fn new() -> Self {
        TextureGenerator {
            perlin: Perlin::new(),
        }
    }

    pub fn generate_liminal_texture(&self, chunk_x: i32, chunk_y: i32) -> Vec<u8> {
        let size = 256usize;
        let mut data = vec![0u8; size * size * 4];

        let seed_x = chunk_x as f64 * 100.0;
        let seed_y = chunk_y as f64 * 100.0;

        for y in 0..size {
            for x in 0..size {
                let idx = (y * size + x) * 4;

                let fx = (x as f64 + seed_x) / 64.0;
                let fy = (y as f64 + seed_y) / 64.0;

                let noise1 = self.perlin.get([fx, fy]);
                let noise2 = self.perlin.get([fx * 0.5, fy * 0.5 + 1000.0]);
                let noise3 = self.perlin.get([fx * 2.0 + 2000.0, fy * 2.0]);

                let combined = (noise1 * 0.5 + noise2 * 0.3 + noise3 * 0.2).abs();
                let value = ((combined * 255.0) as u8).min(255);

                let r = value;
                let g = ((value as f32 * 1.05) as u8).saturating_add(10);
                let b = ((value as f32 * 1.1) as u8).saturating_add(20);

                let grid_x = (x / 32) % 2;
                let grid_y = (y / 32) % 2;
                let grid_val = if (grid_x + grid_y) % 2 == 0 { 10 } else { 5 };

                data[idx] = r.saturating_add(grid_val);
                data[idx + 1] = g.saturating_add(grid_val);
                data[idx + 2] = b.saturating_add(grid_val);
                data[idx + 3] = 255;
            }
        }

        data
    }

    pub fn generate_creepy_details(&self, chunk_x: i32, chunk_y: i32) -> Vec<u8> {
        let size = 256usize;
        let mut data = vec![0u8; size * size * 4];

        let seed_x = chunk_x as f64 * 150.0;
        let seed_y = chunk_y as f64 * 150.0;

        for y in 0..size {
            for x in 0..size {
                let idx = (y * size + x) * 4;

                let fx = (x as f64 + seed_x) / 32.0;
                let fy = (y as f64 + seed_y) / 32.0;

                let noise = self.perlin.get([fx, fy, seed_x * 0.001]);

                if noise > 0.7 {
                    let intensity = ((noise - 0.7) * 255.0 * 1.5) as u8;
                    data[idx] = intensity / 3;
                    data[idx + 1] = intensity / 4;
                    data[idx + 2] = intensity / 5;
                    data[idx + 3] = intensity / 2;
                } else {
                    data[idx + 3] = 0;
                }
            }
        }

        data
    }

    pub fn generate_hazard_zones(&self, chunk_x: i32, chunk_y: i32) -> Vec<u8> {
        let size = 256usize;
        let mut data = vec![0u8; size * size * 4];

        let seed_x = chunk_x as f64 * 200.0;
        let seed_y = chunk_y as f64 * 200.0;

        for y in 0..size {
            for x in 0..size {
                let idx = (y * size + x) * 4;

                let fx = (x as f64 + seed_x) / 100.0;
                let fy = (y as f64 + seed_y) / 100.0;

                let noise = self.perlin.get([fx, fy]);

                if noise > 0.5 {
                    let intensity = ((noise - 0.5) * 255.0 * 2.0) as u8;
                    data[idx] = intensity;
                    data[idx + 1] = 0;
                    data[idx + 2] = 0;
                    data[idx + 3] = intensity / 3;
                } else {
                    data[idx + 3] = 0;
                }
            }
        }

        data
    }
}
