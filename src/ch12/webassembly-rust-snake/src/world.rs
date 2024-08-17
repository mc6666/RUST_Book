mod coord;

use self::coord::Coord;
use rand::Rng;
use std::collections::VecDeque;

const BYTES_PER_PIXEL: u32 = 4;
const START_LEN: i32 = 7;
const INVARIANT: &str = "Snake length > 0"; //todo: this is strange :)

pub struct World {
    pub screen: Screen,
    direction: Coord,
    snake: VecDeque<Coord>,
    alive: bool,
}

impl World {
    pub fn new(width: u32, height: u32) -> World {
        let mut world = World {
            screen: Screen::new(width, height),
            direction: (1, 0).into(),
            snake: VecDeque::new(),
            alive: true,
        };

        world.screen.clear();
        world.create_initial_snake();
        world.create_initial_food();
        world
    }

    pub fn tick(&mut self) {
        if self.alive {
            let new_head = self.get_new_head();
            let new_head_pixel = self.screen.get_color_at(&new_head);

            self.extend_head_to(&new_head);

            match new_head_pixel {
                Color::Food => self.create_food(),
                Color::Snake => self.die(),
                _ => self.shorten_tail(),
            }
        }
    }

    pub fn click(&mut self, x: i32, y: i32) {
        if self.alive {
            let head = self.snake.back().expect(INVARIANT);

            self.direction = match self.direction.x {
                0 => (if x < head.x { -1 } else { 1 }, 0),
                _ => (0, if y < head.y { -1 } else { 1 }),
            }
            .into();
        } else {
            self.reset_game()
        }
    }

    fn reset_game(&mut self) {
        self.direction = (1, 0).into();
        self.snake = VecDeque::new();
        self.alive = true;
        self.screen.clear();
        self.create_initial_snake();
        self.create_initial_food();
    }

    fn create_initial_snake(&mut self) {
        let start_y = self.screen.height as i32 / 2;
        for x in 0..START_LEN {
            self.screen.set_color_at(&(x, start_y).into(), Color::Snake);
            self.snake.push_back((x, start_y).into());
        }
    }

    fn create_initial_food(&mut self) {
        let initial_food_y = self.screen.height as i32 / 2 - 2;
        self.screen
            .set_color_at(&(START_LEN, initial_food_y).into(), Color::Food);
    }

    fn get_new_head(&self) -> Coord {
        let screen_width = self.screen.width;
        let screen_height = self.screen.height;
        let moved_head = *self.snake.back().expect(INVARIANT) + self.direction;
        let x = (moved_head.x + screen_width as i32) % screen_width as i32;
        let y = (moved_head.y + screen_height as i32) % screen_height as i32;
        (x, y).into()
    }

    fn extend_head_to(&mut self, new_head: &Coord) {
        self.screen.set_color_at(new_head, Color::Snake);
        self.snake.push_back(*new_head);
    }

    fn shorten_tail(&mut self) {
        let tail = self.snake.pop_front().expect(INVARIANT);
        self.screen.set_color_at(&tail, Color::Background);
    }

    fn create_food(&mut self) {
        let pixel_count = self.screen.pixel_count as usize;
        let random_skip = rand::thread_rng().gen_range(0..pixel_count) as usize;

        // todo: this will panic if a very good player fills the screen with the snake ... instead a "you are awesome" message should appear :)
        // todo: this is less efficient than the previous solution ... there might be a better way (but still using coords instead of index)

        let coord = self
            .screen
            .iter_pixels()
            .filter(|(color, _)| *color == Color::Background)
            .map(|(_, coord)| coord)
            .collect::<Vec<_>>()
            .into_iter()
            .cycle()
            .skip(random_skip)
            .next()
            .expect("At least one pixel should be free.");

        self.screen.set_color_at(&coord, Color::Food);
    }

    fn die(&mut self) {
        self.alive = false;
        self.screen.set_color_at_edges(Color::Fail);
    }
}

pub struct Screen {
    pub pixel_buffer: Vec<u8>,
    pub pixel_count: u32,
    pub width: u32,
    pub height: u32,
}

impl Screen {
    fn new(width: u32, height: u32) -> Self {
        let pixel_count = width * height;
        let screen_size_in_bytes = pixel_count * BYTES_PER_PIXEL;
        Self {
            pixel_count,
            width,
            height,
            pixel_buffer: vec![255u8; screen_size_in_bytes as usize],
        }
    }

    fn clear(&mut self) {
        self.iter_coords().for_each(|coord| {
            self.set_color_at(&coord, Color::Background);
        });
    }

    fn set_color_at(&mut self, coord: &Coord, color: Color) {
        let i = self.get_buffer_index_for(coord);
        self.pixel_buffer[i..i + 3].copy_from_slice(Rgb::from(&color).as_slice());
    }

    fn set_color_at_edges(&mut self, color: Color) {
        let screen_width = self.width as i32;
        let screen_height = self.height as i32;

        self.iter_coords()
            .filter(|Coord { x, y }| {
                *x == 0 || *y == 0 || *x == screen_width - 1 || *y == screen_height - 1
            })
            .for_each(move |coord| self.set_color_at(&coord, color));
    }

    fn get_color_at(&self, coord: &Coord) -> Color {
        let i = self.get_buffer_index_for(coord);
        (&[
            self.pixel_buffer[i],
            self.pixel_buffer[i + 1],
            self.pixel_buffer[i + 2],
        ])
            .into()
    }

    fn get_buffer_index_for(&self, Coord { x, y }: &Coord) -> usize {
        (*y as usize * self.width as usize + *x as usize) * BYTES_PER_PIXEL as usize
    }

    fn iter_coords(&self) -> impl Iterator<Item = Coord> {
        let width = self.width;
        let height = self.height;
        (0..height as i32).flat_map(move |y| (0..width as i32).map(move |x| (x, y).into()))
    }

    fn iter_pixels(&self) -> impl Iterator<Item = (Color, Coord)> + '_ {
        self.iter_coords()
            .map(|coord: Coord| (self.get_color_at(&coord), coord))
    }
}

type Rgb = [u8; 3];

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Color {
    Background,
    Snake,
    Food,
    Fail,
}

impl From<&Color> for Rgb {
    fn from(color: &Color) -> Self {
        match color {
            Color::Background => [0; 3],
            Color::Snake => [0, 255, 0],
            Color::Food => [0, 0, 255],
            Color::Fail => [255, 0, 0],
        }
    }
}

impl From<&Rgb> for Color {
    fn from(rgb: &Rgb) -> Self {
        match rgb {
            [0, 0, 0] => Color::Background,
            [0, 255, 0] => Color::Snake,
            [0, 0, 255] => Color::Food,
            [255, 0, 0] => Color::Fail,
            _ => panic!("unexpected rgb value"),
        }
    }
}

#[cfg(test)]
mod tests {
    const START_Y: i32 = 15;

    use super::*;

    #[test]
    fn new() {
        let world = World::new(30, 30);

        assert_eq!(
            world.screen.get_color_at(&(0, START_Y).into()),
            Color::Snake
        );
        assert_eq!(
            world.screen.get_color_at(&(START_LEN, START_Y).into()),
            Color::Background
        );
    }

    #[test]
    fn crawl() {
        let mut world = World::new(30, 30);
        world.tick();

        assert_eq!(
            world.screen.get_color_at(&(0, START_Y).into()),
            Color::Background
        );
        assert_eq!(
            world.screen.get_color_at(&(START_LEN, START_Y).into()),
            Color::Snake
        );
    }

    #[test]
    fn turn() {
        let mut world = World::new(30, 30);
        world.click(0, 0);
        world.tick();

        assert_eq!(
            world.screen.get_color_at(&(0, START_Y).into()),
            Color::Background
        );
        assert_eq!(
            world.screen.get_color_at(&(START_LEN, START_Y).into()),
            Color::Background
        );
        assert_eq!(
            world
                .screen
                .get_color_at(&(START_LEN - 1, START_Y - 1).into()),
            Color::Snake
        );
    }

    #[test]
    fn eat() {
        let mut world = World::new(30, 30);
        world.tick();
        world.click(0, 0);
        world.tick();
        world.tick();

        assert_eq!(world.snake.len(), START_LEN as usize + 1);
        assert!(food_exists(&world));
    }

    #[test]
    fn die() {
        let mut world = World::new(30, 30);
        world.click(0, 0);
        world.tick();
        world.click(0, 0);
        world.tick();
        world.click(0, world.screen.height as i32 - 1);
        world.tick();

        assert_eq!(world.screen.get_color_at(&(0, 0).into()), Color::Fail);
    }

    #[test]
    fn stop() {
        let mut world = World::new(30, 30);
        world.click(0, 0);
        world.tick();
        world.click(0, 0);
        world.tick();
        world.click(0, world.screen.height as i32 - 1);
        world.tick();
        world.tick();

        assert_eq!(
            world.screen.get_color_at(&(2, START_Y).into()),
            Color::Snake
        );
    }

    #[test]
    fn revive() {
        let mut world = World::new(30, 30);
        world.click(0, 0);
        world.tick();
        world.click(0, 0);
        world.tick();
        world.click(0, world.screen.height as i32 - 1);
        world.tick();
        world.click(0, 0);

        assert_eq!(world.screen.get_color_at(&(0, 0).into()), Color::Background);
    }

    #[test]
    fn wrap() {
        let mut world = World::new(30, 30);

        for _ in 0..world.screen.width {
            world.tick();
        }

        assert_eq!(
            world.screen.get_color_at(&(0, START_Y).into()),
            Color::Snake
        );
    }

    fn food_exists(world: &World) -> bool {
        world
            .screen
            .iter_pixels()
            .any(|(color, _)| color == Color::Food)
    }
}
