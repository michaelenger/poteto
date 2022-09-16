use crate::rng::Rng;

/// The effect of a roll.
pub enum RollEffect {
    Destiny(i8),
    HurlCost(u8),
    Potatoes(i8),
    Orcs(i8),
}

/// A dice roll.
pub struct Roll {
    pub main_result: u8,
    pub sub_result: u8,
    pub effects: Vec<RollEffect>,
}

impl Roll {
    /// Do a new dice roll.
    pub fn new(rng: &mut Rng) -> Self {
        let main_result = rng.u8(1..7);
        let mut sub_result = rng.u8(1..7);

        let effects = if main_result < 3 {
            // In the Garden...
            match sub_result {
                1 => {
                    vec![RollEffect::Potatoes(1)]
                }
                2 => {
                    vec![RollEffect::Potatoes(1), RollEffect::Destiny(1)]
                }
                3 => {
                    vec![RollEffect::Destiny(1), RollEffect::Orcs(1)]
                }
                4 => {
                    vec![RollEffect::Orcs(1), RollEffect::Potatoes(-1)]
                }
                5 => {
                    vec![RollEffect::Potatoes(-1)]
                }
                6 => {
                    vec![RollEffect::Potatoes(2)]
                }
                0 | 7..=u8::MAX => panic!("how?"),
            }
        } else if main_result < 5 {
            // A Knock at the Door...
            match sub_result {
                1 => {
                    vec![RollEffect::Orcs(1)]
                }
                2 => {
                    vec![RollEffect::Destiny(1)]
                }
                3 => {
                    vec![RollEffect::Orcs(1), RollEffect::Destiny(1)]
                }
                4 => {
                    vec![RollEffect::Potatoes(-1), RollEffect::Orcs(2)]
                }
                5 => {
                    vec![RollEffect::Destiny(1)]
                }
                6 => {
                    vec![RollEffect::Potatoes(2)]
                }
                0 | 7..=u8::MAX => panic!("how?"),
            }
        } else {
            // The world becomes darker...
            sub_result = 0;
            vec![RollEffect::HurlCost(1)]
        };

        Roll {
            main_result,
            sub_result,
            effects,
        }
    }
}
