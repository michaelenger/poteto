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
    pub description: String,
}

impl Roll {
    /// Do a new dice roll.
    pub fn new(rng: &mut Rng) -> Self {
        let main_result = rng.u8(1..7);
        let mut sub_result = rng.u8(1..7);

        #[allow(unused_assignments)]
        let mut description = "";

        let effects = if main_result < 3 {
            // In the Garden...
            match sub_result {
                1 => {
                    description = "You happily root\nabout all day in\nyour garden.";
                    vec![RollEffect::Potatoes(1)]
                }
                2 => {
                    description = "You narrowly\navoid a visitor\nby hiding in a\npotato sack.";
                    vec![RollEffect::Potatoes(1), RollEffect::Destiny(1)]
                }
                3 => {
                    description = "A hooded\nstranger lingers\noutside your\nfarm.";
                    vec![RollEffect::Destiny(1), RollEffect::Orcs(1)]
                }
                4 => {
                    description = "Your field is\nravaged in the\nnight by unseen\nenemies.";
                    vec![RollEffect::Orcs(1), RollEffect::Potatoes(-1)]
                }
                5 => {
                    description = "You trade\npotatoes for\nother delicious\nfoodstuffs.";
                    vec![RollEffect::Potatoes(-1)]
                }
                6 => {
                    description = "You burrow into\na bumper crop of\npotatoes. Do you\ncry with joy?\nPossibly.";
                    vec![RollEffect::Potatoes(2)]
                }
                0 | 7..=u8::MAX => panic!("how?"),
            }
        } else if main_result < 5 {
            // A Knock at the Door...
            match sub_result {
                1 => {
                    description = "A distant cousin\ncomes by. They\nare after your\npotatoes. They\nmay snitch on\nyou.";
                    vec![RollEffect::Orcs(1)]
                }
                2 => {
                    description = "A dwarven\nstranger stops\nby. You refuse\nthem entry.\nGhastly\ncreatures.";
                    vec![RollEffect::Destiny(1)]
                }
                3 => {
                    description = "A wizard strolls\nby. You\npointedly draw\nthe curtains.";
                    vec![RollEffect::Orcs(1), RollEffect::Destiny(1)]
                }
                4 => {
                    description =
                        "There are\nrumours of war\nin the reaches.\n\nYou eat some\npotatoes.";
                    vec![RollEffect::Potatoes(-1), RollEffect::Orcs(2)]
                }
                5 => {
                    description = "You spot an elf\nskulking about.\nThey are not\nserious people.";
                    vec![RollEffect::Destiny(1)]
                }
                6 => {
                    description = "It's a sack of\npotatoes from a\ngenerous\nneighbour. You\nreally must\nremember to pay\nthem a visit one\nof these years.";
                    vec![RollEffect::Potatoes(2)]
                }
                0 | 7..=u8::MAX => panic!("how?"),
            }
        } else {
            description = "The world grows\never darker...";
            sub_result = 0;
            vec![RollEffect::HurlCost(1)]
        };

        Roll {
            main_result,
            sub_result,
            effects,
            description: description.into(),
        }
    }
}
