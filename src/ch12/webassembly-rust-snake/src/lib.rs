mod world;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};
use world::World;

const TICK_MILLISECONDS: u32 = 75;

#[wasm_bindgen]
pub struct Game {
    world: World,
    elapsed_milliseconds: u32,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game {
            world: World::new(30, 30),
            elapsed_milliseconds: 0,
        }
    }

    pub fn tick(&mut self, elapsed_milliseconds: u32) {
        self.elapsed_milliseconds += elapsed_milliseconds;

        if self.elapsed_milliseconds >= TICK_MILLISECONDS {
            self.elapsed_milliseconds = 0;
            self.world.tick();
        }
    }

    pub fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.world.screen.pixel_buffer),
            self.world.screen.width,
            self.world.screen.height,
        )
        .expect("should create ImageData from array");

        ctx.put_image_data(&data, 0.0, 0.0)
            .expect("should write array to context");
    }

    pub fn width(&self) -> u32 {
        self.world.screen.width
    }

    pub fn height(&self) -> u32 {
        self.world.screen.height
    }

    pub fn click(&mut self, x: i32, y: i32) {
        self.world.click(x, y);
    }
}
