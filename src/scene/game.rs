use std::path::Path;

use sdl2::event::Event;
use sdl2::image::LoadTexture; // Or INIT_JPG
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::render::Renderer;

use ::Message;
use scene::Scene;
use actor::{Player, Cat, SpawnBuilding, TargetBuilding};

pub struct Game {
	player: Player,
	cats: [Cat; 4],
	spawner: SpawnBuilding,
	target: TargetBuilding
}

impl Game {
	pub fn new() -> Game {
		Game {
			player: Player::new(),
			cats: [Cat::new(), Cat::new(), Cat::new(), Cat::new()],
			spawner: SpawnBuilding::new(),
			target: TargetBuilding::new()
		}
	}
}

impl Scene for Game {
	fn update(&mut self, renderer: &mut Renderer, deltaTime: f32) {

	}

	fn render(&self, renderer: &mut Renderer) {
		// Clear the area.
		renderer.set_draw_color(Color::RGBA(0,0,0,255));
		renderer.clear();

		renderer.present();
	}

	fn destroy(&mut self) {
	}

	fn get_messages(&mut self) -> Message {
		Message::None
	}

	fn handle_event(&mut self, event: &Event) {
		match *event {
			Event::KeyDown {keycode: Some(Keycode::A), ..} | Event::KeyDown{keycode: Some(Keycode::Left), ..} => {
				println!("Move left.");
			},
			_ => {}
		};
	}
}
