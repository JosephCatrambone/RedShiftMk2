
pub mod game;

use sdl2::render::Renderer;

use ::Message;

pub trait Scene {
	fn update(&mut self, renderer: &Renderer, deltaTime: f32);
	fn render(&self, renderer: &Renderer);
	fn destroy(&mut self);
	fn get_messages(&mut self) -> Message;
}
