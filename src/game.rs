use crate::input::{ButtonState, Gamepad};
use crate::minifont;
use crate::palette::set_colors;
use crate::rng::Rng;
use crate::roll::{Roll, RollEffect};
use crate::sprites;
use crate::wasm4;

/// Convert an amount to text with a + or - in front.
fn amount_text(amount: &i8) -> String {
    if *amount > 0 {
        format!("+{}", amount)
    } else {
        format!("{}", amount)
    }
}

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
        set_colors(0x0431);
        sprites::logo(5, 5);

        set_colors(0x20);
        minifont::draw("Based on the RPG".into(), 122, 16);
        minifont::draw("by Oliver Darkshire".into(), 114, 22);

        // Headers
        set_colors(0x04);
        wasm4::text("Destiny", 5, 36);
        wasm4::text("Potatoes", 5, 66);
        wasm4::text("Orcs", 5, 96);

        // Pips
        self.draw_pips(self.destiny, 5, 46);
        self.draw_pips(self.potatoes, 5, 76);
        self.draw_pips(self.orcs, 5, 106);

        // Buttons
        self.draw_button(
            "(X)ROLL",
            10,
            130,
            self.gamepad.one == ButtonState::Held || self.gamepad.one == ButtonState::Pressed,
        );

        self.draw_button(
            "(Z)HURL",
            90,
            130,
            self.gamepad.two == ButtonState::Held || self.gamepad.two == ButtonState::Pressed,
        );

        // Day counter
        set_colors(0x21);
        minifont::draw(format!("Day:{:02}", self.day_counter), 40, 142);

        // Hurl data
        set_colors(0x21);
        minifont::draw(format!("-{} potatoes", self.hurl_cost), 117, 142);
        minifont::draw("-1 orcs".into(), 117, 148);

        if self.is_game_over() {
            self.draw_overlay();

            // Text
            if self.destiny >= 10 {
                self.draw_title("Your Destiny", 30, 20);
                self.draw_title("Awaits", 56, 32);
                set_colors(0x4);
                wasm4::text("An interfering\nbard or wizard\nturns up at your\ndoorstep with a\nquest, and you\nare whisked away\nagainst your\nwill on an\nadventure.", 16, 50);
            } else if self.potatoes >= 10 {
                self.draw_title("Fully Stocked", 26, 20);
                set_colors(0x4);
                wasm4::text("You have enough\npotatoes that\nyou can go\nunderground and\nnot return to\nthe surface\nuntil the danger\nis past. You\nnestle down in\nyour burrow and\nenjoy your well\nearned rest.", 16, 34);
            } else if self.orcs >= 10 {
                self.draw_title("The Orcish", 40, 20);
                self.draw_title("Horde Arrives!", 26, 32);
                set_colors(0x4);
                wasm4::text("The orcs finally\nfind your potato\nfarm. Alas, orcs\nare not so\ninterested in\npotatoes as they\nare in eating\nyou, and you end\nup in a cookpot.", 16, 50);
            }

            // Retry button
            self.draw_button("(R)RETRY", 82, 138, false);
        } else if let Some(roll) = &self.roll {
            self.draw_overlay();

            // Dice
            sprites::die(roll.main_result, 62, 20);
            sprites::die(roll.sub_result, 82, 20);

            // Description text
            set_colors(0x4);
            wasm4::text(&roll.description, 16, 44);

            // Effect text
            set_colors(0x21);
            let mut line = 0;
            for effect in &roll.effects {
                let text = match effect {
                    RollEffect::Destiny(num) => {
                        format!("{} destiny", amount_text(num))
                    }
                    RollEffect::HurlCost(num) => {
                        format!("{} cost to hurl", amount_text(&(*num as i8)))
                    }
                    RollEffect::Potatoes(num) => {
                        format!("{} potatoes", amount_text(num))
                    }
                    RollEffect::Orcs(num) => {
                        format!("{} orcs", amount_text(num))
                    }
                };

                minifont::draw(text, 80, 114 + (line * 8));

                line += 1;
            }

            // Continue button
            self.draw_button(
                "(X)CONTINUE",
                58,
                138,
                self.gamepad.one == ButtonState::Held || self.gamepad.one == ButtonState::Pressed,
            );
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

    fn draw_overlay(&self) {
        // Overlay
        set_colors(0x4);
        wasm4::rect(12, 12, 140, 140);
        set_colors(0x21);
        wasm4::rect(10, 10, 140, 140);
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

    fn draw_title(&self, text: &str, x: i32, y: i32) {
        set_colors(0x04);
        wasm4::text(text, x + 1, y + 1);
        set_colors(0x03);
        wasm4::text(text, x, y);
    }

    fn roll_or_effect(&mut self) {
        match &self.roll {
            Some(roll) => {
                for effect in &roll.effects {
                    match effect {
                        RollEffect::Destiny(amount) => {
                            if *amount < 0 && self.destiny == 0 {
                                continue;
                            }
                            let val: i8 = (self.destiny as i8) + (*amount as i8);
                            self.destiny = val as u8;
                        }
                        RollEffect::HurlCost(amount) => {
                            self.hurl_cost += *amount;
                        }
                        RollEffect::Potatoes(amount) => {
                            if *amount < 0 && self.potatoes == 0 {
                                continue;
                            }
                            let val: i8 = (self.potatoes as i8) + (*amount as i8);
                            self.potatoes = val as u8;
                        }
                        RollEffect::Orcs(amount) => {
                            if *amount < 0 && self.orcs == 0 {
                                continue;
                            }
                            let val: i8 = (self.orcs as i8) + (*amount as i8);
                            self.orcs = val as u8;
                        }
                    }
                }

                self.roll = None;
            }
            None => {
                self.day_counter += 1;
                self.roll = Some(Roll::new(&mut self.rng));
            }
        }
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
                self.roll_or_effect();
            }
            if self.gamepad.two == ButtonState::Released {
                self.hurl();
            }
        }

        self.draw();
    }
}
