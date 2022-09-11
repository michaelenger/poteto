use crate::wasm4;

// Set the draw colors.
pub fn set_colors(colors: u16) {
	unsafe {
		*wasm4::DRAW_COLORS = colors.into();
	}
}

// Set the active color palette.
pub fn set_palette(palette: [u32; 4]) {
	unsafe {
		*wasm4::PALETTE = palette;
	}
}
