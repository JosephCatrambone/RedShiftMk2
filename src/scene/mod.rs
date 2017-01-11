
pub mod game;

use sdl2::event::Event;
use sdl2::render::Renderer;

use ::Message;

pub trait Scene {
	fn update(&mut self, renderer: &mut Renderer, deltaTime: f32);
	fn render(&self, renderer: &mut Renderer);
	fn destroy(&mut self);
	fn handle_event(&mut self, &Event);
	fn get_messages(&mut self) -> Message;
}
