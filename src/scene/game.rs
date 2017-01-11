
use sdl2::render::Renderer;

use ::Message;
use scene::Scene;

pub struct Game {
}

impl Game {
	pub fn new() -> Game {
		Game {
		}
	}
}

impl Scene for Game {
	fn update(&mut self, renderer: &Renderer, deltaTime: f32) {
	}

	fn render(&self, renderer: &Renderer) {
	}

	fn destroy(&mut self) {
	}

	fn get_messages(&mut self) -> Message {
		Message::None
	}
}
