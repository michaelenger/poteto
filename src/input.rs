use crate::wasm4;

#[derive(PartialEq)]
pub enum ButtonState {
    Inactive,
    Pressed,
    Held,
    Released,
}

pub struct Gamepad {
    previous_gamepad: u8,
    pub up: ButtonState,
    pub down: ButtonState,
    pub left: ButtonState,
    pub right: ButtonState,
    pub one: ButtonState,
    pub two: ButtonState,
}

impl Gamepad {
    pub fn new() -> Self {
        Gamepad {
            previous_gamepad: 0,
            up: ButtonState::Inactive,
            down: ButtonState::Inactive,
            left: ButtonState::Inactive,
            right: ButtonState::Inactive,
            one: ButtonState::Inactive,
            two: ButtonState::Inactive,
        }
    }

    pub fn update(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let pressed = gamepad & (gamepad ^ self.previous_gamepad);
        let released = self.previous_gamepad & (self.previous_gamepad ^ gamepad);

        self.up = {
            if wasm4::BUTTON_UP & pressed != 0 {
                ButtonState::Pressed
            } else if wasm4::BUTTON_UP & released != 0 {
                ButtonState::Released
            } else if wasm4::BUTTON_UP & gamepad != 0 {
                ButtonState::Held
            } else {
                ButtonState::Inactive
            }
        };

        self.down = {
            if wasm4::BUTTON_DOWN & pressed != 0 {
                ButtonState::Pressed
            } else if wasm4::BUTTON_DOWN & released != 0 {
                ButtonState::Released
            } else if wasm4::BUTTON_DOWN & gamepad != 0 {
                ButtonState::Held
            } else {
                ButtonState::Inactive
            }
        };

        self.left = {
            if wasm4::BUTTON_LEFT & pressed != 0 {
                ButtonState::Pressed
            } else if wasm4::BUTTON_LEFT & released != 0 {
                ButtonState::Released
            } else if wasm4::BUTTON_LEFT & gamepad != 0 {
                ButtonState::Held
            } else {
                ButtonState::Inactive
            }
        };

        self.right = {
            if wasm4::BUTTON_RIGHT & pressed != 0 {
                ButtonState::Pressed
            } else if wasm4::BUTTON_RIGHT & released != 0 {
                ButtonState::Released
            } else if wasm4::BUTTON_RIGHT & gamepad != 0 {
                ButtonState::Held
            } else {
                ButtonState::Inactive
            }
        };

        self.one = {
            if wasm4::BUTTON_1 & pressed != 0 {
                ButtonState::Pressed
            } else if wasm4::BUTTON_1 & released != 0 {
                ButtonState::Released
            } else if wasm4::BUTTON_1 & gamepad != 0 {
                ButtonState::Held
            } else {
                ButtonState::Inactive
            }
        };

        self.two = {
            if wasm4::BUTTON_2 & pressed != 0 {
                ButtonState::Pressed
            } else if wasm4::BUTTON_2 & released != 0 {
                ButtonState::Released
            } else if wasm4::BUTTON_2 & gamepad != 0 {
                ButtonState::Held
            } else {
                ButtonState::Inactive
            }
        };

        self.previous_gamepad = gamepad;
    }
}
