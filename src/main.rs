extern crate sdl2;

use std::env;
use std::time::Duration;
use std::path::Path;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{LoadTexture, INIT_PNG}; // Or INIT_JPG
use sdl2::rect::{Rect, Point};

mod scene;
mod actor;

use scene::Scene;
use scene::game::Game;
use actor::*;

pub enum Message {
	RequestSceneChange(Box<Scene>),
	RequestQuit,
	None
}

fn main() {
	let args: Vec<_> = env::args().collect(); // &args[0] is program name.
	// Initialize engine.
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let image_context = sdl2::image::init(INIT_PNG).unwrap();
	let window = video_subsystem.window("Title!", 640, 480).position_centered().build().unwrap();
	let mut renderer = window.renderer().accelerated().build().unwrap();
	let mut timer = sdl_context.timer().unwrap();
	let mut event_pump = sdl_context.event_pump().unwrap();
	let mut scene_stack = Vec::<Box<Scene>>::new();

	scene_stack.push(Box::new(Game::new(&mut renderer)));

	// Sorta' standard way to load.
	// let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/blah.bmp")).unwrap()
	// let texture = renderer.create_texture_from_surface(&temp_surface).unwrap()
	// Alternative way:
	// let texture = renderer.load_texture(Path::new("assets/blah.png")).unwrap();

	// Jump into main loop.
	'mainloop: loop {
		let mut current_scene = scene_stack.last_mut().unwrap();
		// Handle input
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
					break 'mainloop;
				},
				_ => {}
			};
			current_scene.handle_event(&event);
		}
		// Update logi
		current_scene.update(&mut renderer, 16.0f32);
		// Render.
		current_scene.render(&mut renderer);
		// Delay before next frame.
		std::thread::sleep(Duration::from_millis(16));
	}
}
