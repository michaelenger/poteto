use crate::input::{ButtonState, Gamepad};
use crate::palette::set_colors;
use crate::rng::Rng;
use crate::wasm4;

pub struct Game {
    destiny: u32,
    potatoes: u32,
    orcs: u32,
    main_result: u8,
    sub_result: u8,
    hurl_cost: u8,
    gamepad: Gamepad,
    rng: Rng,
}

impl Game {
    pub fn new() -> Self {
        Game {
            destiny: 0,
            potatoes: 0,
            orcs: 0,
            main_result: 0,
            sub_result: 0,
            hurl_cost: 1,
            gamepad: Gamepad::new(),
            rng: Rng::new(),
        }
    }

    fn draw(&self) {
        // Title
        set_colors(0x04);
        wasm4::text("POTE~TO", 6, 6);
        set_colors(0x03);
        wasm4::text("POTE~TO", 5, 5);

        // Headers
        set_colors(0x04);
        wasm4::text("Destiny", 5, 30);
        wasm4::text("Potatoes", 5, 60);
        wasm4::text("Orcs", 5, 90);

        // Pips
        set_colors(0x03);
        self.draw_pips(self.destiny, 5, 40);
        self.draw_pips(self.potatoes, 5, 70);
        self.draw_pips(self.orcs, 5, 100);

        if self.is_game_over() {
            set_colors(0x04);
            wasm4::text("GAME OVER", 43, 131);
            set_colors(0x03);
            wasm4::text("GAME OVER", 42, 130);
        } else {
            // Buttons
            if self.gamepad.one == ButtonState::Held || self.gamepad.one == ButtonState::Pressed {
                set_colors(0x04);
                wasm4::text("(X)ROLL", 10, 131);
            } else {
                set_colors(0x02);
                wasm4::text("(X)ROLL", 10, 131);
                set_colors(0x04);
                wasm4::text("(X)ROLL", 10, 130);
            }

            if self.gamepad.two == ButtonState::Held || self.gamepad.two == ButtonState::Pressed {
                set_colors(0x04);
                wasm4::text("(Z)HURL", 90, 131);
            } else {
                set_colors(0x02);
                wasm4::text("(Z)HURL", 90, 131);
                set_colors(0x04);
                wasm4::text("(Z)HURL", 90, 130);
            }

            // Roll and hurl data
            set_colors(0x04);
            wasm4::text(format!("{} {}", self.main_result, self.sub_result), 28, 142);
            wasm4::text(format!("{}", self.hurl_cost), 118, 142);
        }
    }

    fn draw_pips(&self, num: u32, x: i32, y: i32) {
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

    fn roll(&mut self) {
        self.main_result = self.rng.u8(1..7);
        self.sub_result = self.rng.u8(1..7);

        if self.main_result < 3 {
            // In the Garden...
            match self.sub_result {
                1 => {
                    self.potatoes += 1;
                }
                2 => {
                    self.potatoes += 1;
                    self.destiny += 1;
                }
                3 => {
                    self.destiny += 1;
                    self.orcs += 1;
                }
                4 => {
                    self.orcs += 1;
                    if self.potatoes > 0 {
                        self.potatoes -= 1;
                    }
                }
                5 => {
                    if self.potatoes > 0 {
                        self.potatoes -= 1;
                    }
                }
                6 => {
                    self.potatoes += 2;
                }
                0 | 7..=u8::MAX => panic!("how?"),
            }
        } else if self.main_result < 5 {
            // A Knock at the Door...
            match self.sub_result {
                1 => {
                    self.orcs += 1;
                }
                2 => {
                    self.destiny += 1;
                }
                3 => {
                    self.orcs += 1;
                    self.destiny += 1;
                }
                4 => {
                    if self.potatoes > 0 {
                        self.potatoes -= 1;
                    }
                    self.orcs += 2;
                }
                5 => {
                    self.destiny += 1;
                }
                6 => {
                    self.potatoes += 2;
                }
                0 | 7..=u8::MAX => panic!("how?"),
            }
        } else if self.main_result == 5 {
            // The world becomes darker...
            self.hurl_cost += 1
        } else {
            // The world becomes brighter...
            if self.hurl_cost > 1 {
                self.hurl_cost -= 1;
            }
        }
    }

    fn hurl(&mut self) {
        if self.orcs > 0 && self.hurl_cost as u32 <= self.potatoes {
            self.potatoes -= self.hurl_cost as u32;
            self.orcs -= 1;
        }
    }

    fn is_game_over(&self) -> bool {
        self.destiny > 9 || self.potatoes > 9 || self.orcs > 9
    }

    pub fn update(&mut self) {
        self.gamepad.update();

        if !self.is_game_over() {
            self.rng.tick();

            if self.gamepad.one == ButtonState::Released {
                self.roll();
            }
            if self.gamepad.two == ButtonState::Released {
                self.hurl();
            }
        }

        self.draw();
    }
}
