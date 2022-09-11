use crate::palette::set_colors;
use crate::wasm4;

pub struct Game {
	destiny: u32,
	potatoes: u32,
	orcs: u32,
}

impl Game {
	pub fn new() -> Self {
		Game {
			destiny: 1,
			potatoes: 2,
			orcs: 3,
		}
	}

	pub fn update(&mut self) {
		//self.input();

		// other updates

		//self.draw();
	}
}
