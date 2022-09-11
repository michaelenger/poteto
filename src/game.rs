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

	fn draw(&self) {
		// Title
		set_colors(0x04);
		wasm4::text("POTE~TO", 6, 6);
		set_colors(0x03);
		wasm4::text("POTE~TO", 5, 5);

		set_colors(0x04);
		wasm4::text("Destiny", 5, 30);
		wasm4::text("Potatoes", 5, 60);
		wasm4::text("Orcs", 5, 90);

		set_colors(0x03);
		self.draw_pips(self.destiny, 5, 40);
		self.draw_pips(self.potatoes, 5, 70);
		self.draw_pips(self.orcs, 5, 100);

		set_colors(0x02);
		wasm4::text("(X)ROLL", 10, 141);
		wasm4::text("(Z)HURL", 90, 141);

		set_colors(0x04);
		wasm4::text("(X)ROLL", 10, 140);
		wasm4::text("(Z)HURL", 90, 140);
	}

	fn draw_pips(&self, num: u32, x:i32, y:i32) {
		let size: u32 = 14;

		for i in 0..10 {
			let offset = (size + 1) * i;

			if i < num {
				set_colors(0x03);
			} else {
				set_colors(0x30);
			}

			wasm4::rect(x + (offset as i32), y, size, size);
		}
	}

	fn input(&mut self) {
		//
	}

	pub fn update(&mut self) {
		self.input();

		// other updates

		self.draw();
	}
}
