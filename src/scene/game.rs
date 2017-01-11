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
	target: Actor,

	player_acceleration: f32
}

impl Game {
	pub fn new(renderer: &mut Renderer) -> Game {
		// Load the textures
		let player_texture = renderer.load_texture(Path::new("assets/player.png")).unwrap();
		let building_texture1 = renderer.load_texture(Path::new("assets/building1.png")).unwrap();
		let building_texture2 = renderer.load_texture(Path::new("assets/building2.png")).unwrap();

		let mut g = Game {
			player: Actor::new(player_texture),
			cats: Vec::new(),
			spawner: Actor::new(building_texture1),
			target: Actor::new(building_texture2),
			player_acceleration: 2.0f32
		};

		g.player.max_speed = 6.0f32;
		g
	}
}

impl Scene for Game {
	fn update(&mut self, renderer: &mut Renderer, delta_time: f32) {
		self.player.update(delta_time);
		self.spawner.update(delta_time);
		self.target.update(delta_time);

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
			// Left
			Event::KeyDown {keycode: Some(Keycode::A), ..} | Event::KeyDown{keycode: Some(Keycode::Left), ..} => {
				self.player.acceleration[0] -= self.player_acceleration;
			},
			Event::KeyUp {keycode: Some(Keycode::A), ..} | Event::KeyUp{keycode: Some(Keycode::Left), ..} => {
				self.player.acceleration[0] = 0f32;
				self.player.velocity[0] = 0f32;
			},
			// Right
			Event::KeyDown {keycode: Some(Keycode::D), ..} | Event::KeyDown{keycode: Some(Keycode::Right), ..} => {
				self.player.acceleration[0] += self.player_acceleration;
			},
			Event::KeyUp {keycode: Some(Keycode::D), ..} | Event::KeyUp{keycode: Some(Keycode::Right), ..} => {
				self.player.acceleration[0] = 0f32;
				self.player.velocity[0] = 0f32;
			},
			// Debug
			Event::KeyUp {keycode: Some(Keycode::U), ..} => {
				self.player.max_speed += 0.1f32;
				println!("Player max speed: {}", self.player.max_speed);
			},
			Event::KeyUp {keycode: Some(Keycode::I), ..} => {
				self.player.max_speed -= 0.1f32;
				println!("Player max speed: {}", self.player.max_speed);
			},
			Event::KeyUp {keycode: Some(Keycode::O), ..} => {
				self.player_acceleration += 0.1f32;
				println!("Player accel: {}", self.player_acceleration);
			},
			Event::KeyUp {keycode: Some(Keycode::P), ..} => {
				self.player_acceleration -= 0.1f32;
				println!("Player accel: {}", self.player_acceleration);
			},


			_ => {}
		};
	}
}
