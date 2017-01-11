
use sdl2::render::{Texture, Renderer};
use sdl2::rect::{Rect, Point};

pub struct Actor {
	position : [f32; 2],
	velocity : [f32; 2],
	acceleration : [f32; 2],
	rotation : f32,
	angular_velocity : f32,
	texture: Texture,
}

impl Actor {
	pub fn new(sprite: Texture) -> Self {
		Actor {
			position: [0f32, 0f32],
			velocity: [0f32, 0f32],
			acceleration: [0f32, 0f32],
			rotation: 0f32,
			angular_velocity: 0f32,
			texture: sprite

		}
	}

	pub fn update(&mut self, delta_time: f32) {
		self.position[0] += self.velocity[0]*delta_time;
		self.position[1] += self.velocity[1]*delta_time;
		self.velocity[0] += self.acceleration[0]*delta_time;
		self.velocity[1] += self.acceleration[1]*delta_time;
		self.rotation += self.angular_velocity*delta_time;
	}

	pub fn render(&self, renderer: &mut Renderer) {
		renderer.copy_ex(&self.texture, None, Some(Rect::new(self.position[0] as i32, self.position[1] as i32, self.texture.query().width, self.texture.query().height)), self.rotation as f64, None, false, false);
	}
}