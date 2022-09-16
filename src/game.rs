use crate::input::{ButtonState, Gamepad};
use crate::minifont;
use crate::palette::set_colors;
use crate::rng::Rng;
use crate::roll::{Roll, RollEffect};
use crate::sprites;
use crate::wasm4;

pub struct Game {
    destiny: u8,
    potatoes: u8,
    orcs: u8,
    hurl_cost: u8,
    day_counter: u8,
    gamepad: Gamepad,
    rng: Rng,
    roll: Option<Roll>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            destiny: 0,
            potatoes: 0,
            orcs: 0,
            hurl_cost: 1,
            day_counter: 1,
            gamepad: Gamepad::new(),
            rng: Rng::new(),
            roll: None,
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
        self.draw_pips(self.destiny, 5, 40);
        self.draw_pips(self.potatoes, 5, 70);
        self.draw_pips(self.orcs, 5, 100);

        // Day counter
        set_colors(0x03);
        wasm4::text(format!("Day:{:02}", self.day_counter), 106, 5);

        if self.is_game_over() {
            set_colors(0x04);
            wasm4::text("GAME OVER", 43, 131);
            set_colors(0x03);
            wasm4::text("GAME OVER", 42, 130);
        } else {
            // Buttons
            self.draw_button(
                "(X)ROLL",
                10,
                128,
                self.gamepad.one == ButtonState::Held || self.gamepad.one == ButtonState::Pressed,
            );

            self.draw_button(
                "(Z)HURL",
                90,
                128,
                self.gamepad.two == ButtonState::Held || self.gamepad.two == ButtonState::Pressed,
            );

            // Dice
            match &self.roll {
                Some(roll) => {
                    sprites::die(roll.main_result, 21, 138);
                    sprites::die(roll.sub_result, 41, 138);
                }
                None => {}
            }

            // Hurl data
            set_colors(0x21);
            minifont::draw(format!("-{} potatoes", self.hurl_cost), 117, 140);
            minifont::draw("-1 orcs".into(), 117, 146);
        }
    }

    fn draw_button(&self, text: &str, x: i32, y: i32, down: bool) {
        if down {
            set_colors(0x04);
            wasm4::text(text, x, y + 1);
        } else {
            set_colors(0x02);
            wasm4::text(text, x, y + 1);
            set_colors(0x04);
            wasm4::text(text, x, y);
        }
    }

    fn draw_pips(&self, num: u8, x: i32, y: i32) {
        let size: u32 = 14;

        for i in 0..10_u8 {
            let offset = (size + 1) * (i as u32);

            if i < num {
                set_colors(0x03);
            } else {
                set_colors(0x30);
            }

            wasm4::rect(x + (offset as i32), y, size, size);
        }
    }

    fn roll(&mut self) {
        self.day_counter += 1;
        let roll = Roll::new(&mut self.rng);

        for effect in &roll.effects {
            match effect {
                RollEffect::Destiny(amount) => {
                    if *amount < 0 && self.destiny == 0 {
                        break;
                    }
                    let val: i8 = (self.destiny as i8) + (*amount as i8);
                    self.destiny = val as u8;
                }
                RollEffect::HurlCost(amount) => {
                    self.hurl_cost += *amount;
                }
                RollEffect::Potatoes(amount) => {
                    if *amount < 0 && self.potatoes == 0 {
                        break;
                    }
                    let val: i8 = (self.potatoes as i8) + (*amount as i8);
                    self.potatoes = val as u8;
                }
                RollEffect::Orcs(amount) => {
                    if *amount < 0 && self.orcs == 0 {
                        break;
                    }
                    let val: i8 = (self.orcs as i8) + (*amount as i8);
                    self.orcs = val as u8;
                }
            }
        }

        self.roll = Some(roll);
    }

    fn hurl(&mut self) {
        if self.orcs > 0 && self.hurl_cost <= self.potatoes {
            self.potatoes -= self.hurl_cost;
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
