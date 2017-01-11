use std::path::Path;

use sdl2::event::Event;
use sdl2::image::LoadTexture; // Or INIT_JPG
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::render::Renderer;

use ::Message;
use scene::Scene;
use actor::Actor;

pub struct Game {
	player: Actor,
	cats: Vec<Actor>,
	spawner: Actor,
	target: Actor
}

impl Game {
	pub fn new(renderer: &mut Renderer) -> Game {
		// Load the textures
		let player_texture = renderer.load_texture(Path::new("assets/player.png")).unwrap();
		let building_texture1 = renderer.load_texture(Path::new("assets/building1.png")).unwrap();
		let building_texture2 = renderer.load_texture(Path::new("assets/building2.png")).unwrap();

		Game {
			player: Actor::new(player_texture),
			cats: Vec::new(),
			spawner: Actor::new(building_texture1),
			target: Actor::new(building_texture2)
		}
	}
}

impl Scene for Game {
	fn update(&mut self, renderer: &mut Renderer, delta_time: f32) {

	}

	fn render(&self, mut renderer: &mut Renderer) {
		// Clear the area.
		renderer.set_draw_color(Color::RGBA(0,0,0,255));
		renderer.clear();

		self.player.render(&mut renderer);
		/*
		for cat in &self.cats {
			cat.render(&mut renderer);
		}
		*/
		self.spawner.render(&mut renderer);
		self.target.render(&mut renderer);

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
