use crate::palette::set_colors;
use crate::wasm4;

const DIE_1: [u8; 32] = [
    0x3f, 0xfc, 0x7f, 0xfe, 0xe0, 0x07, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc1, 0x83,
    0xc1, 0x83, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xe0, 0x07, 0x7f, 0xfe, 0x3f, 0xfc,
];
const DIE_2: [u8; 32] = [
    0x3f, 0xfc, 0x7f, 0xfe, 0xe0, 0x07, 0xc0, 0x03, 0xcc, 0x03, 0xcc, 0x03, 0xc0, 0x03, 0xc0, 0x03,
    0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x33, 0xc0, 0x33, 0xc0, 0x03, 0xe0, 0x07, 0x7f, 0xfe, 0x3f, 0xfc,
];
const DIE_3: [u8; 32] = [
    0x3f, 0xfc, 0x7f, 0xfe, 0xe0, 0x07, 0xc0, 0x03, 0xcc, 0x03, 0xcc, 0x03, 0xc0, 0x03, 0xc1, 0x83,
    0xc1, 0x83, 0xc0, 0x03, 0xc0, 0x33, 0xc0, 0x33, 0xc0, 0x03, 0xe0, 0x07, 0x7f, 0xfe, 0x3f, 0xfc,
];
const DIE_4: [u8; 32] = [
    0x3f, 0xfc, 0x7f, 0xfe, 0xe0, 0x07, 0xc0, 0x03, 0xcc, 0x33, 0xcc, 0x33, 0xc0, 0x03, 0xc0, 0x03,
    0xc0, 0x03, 0xc0, 0x03, 0xcc, 0x33, 0xcc, 0x33, 0xc0, 0x03, 0xe0, 0x07, 0x7f, 0xfe, 0x3f, 0xfc,
];
const DIE_5: [u8; 32] = [
    0x3f, 0xfc, 0x7f, 0xfe, 0xe0, 0x07, 0xc0, 0x03, 0xcc, 0x33, 0xcc, 0x33, 0xc0, 0x03, 0xc1, 0x83,
    0xc1, 0x83, 0xc0, 0x03, 0xcc, 0x33, 0xcc, 0x33, 0xc0, 0x03, 0xe0, 0x07, 0x7f, 0xfe, 0x3f, 0xfc,
];
const DIE_6: [u8; 32] = [
    0x3f, 0xfc, 0x7f, 0xfe, 0xe0, 0x07, 0xc0, 0x03, 0xcc, 0x33, 0xcc, 0x33, 0xc0, 0x03, 0xcc, 0x33,
    0xcc, 0x33, 0xc0, 0x03, 0xcc, 0x33, 0xcc, 0x33, 0xc0, 0x03, 0xe0, 0x07, 0x7f, 0xfe, 0x3f, 0xfc,
];

pub fn die(value: u8, x: i32, y: i32) {
    set_colors(0x41);
    let sprite = match value {
        0 => {
            set_colors(0x21);
            &DIE_6
        }
        1 => &DIE_1,
        2 => &DIE_2,
        3 => &DIE_3,
        4 => &DIE_4,
        5 => &DIE_5,
        6 => &DIE_6,
        7..=u8::MAX => panic!("how?"),
    };

    wasm4::blit(sprite, x, y, 16, 16, wasm4::BLIT_1BPP);
}